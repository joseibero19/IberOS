use crate::{println, pause_execution};
use crate::string::String;
use crate::vga_buffer::{self, Color};
use spin::Mutex;
use lazy_static::lazy_static;
use core::sync::atomic::{AtomicUsize, Ordering};

// Simple process ID counter
static NEXT_PID: AtomicUsize = AtomicUsize::new(1);

// Message structure for IPC
#[derive(Debug, Clone, Copy)]
pub struct Message {
    sender_pid: usize,
    receiver_pid: usize,
    message_type: MessageType,
    data: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    Command,
    Response,
    Notification,
}

// Simple process structure
pub struct Process {
    pid: usize,
    name: &'static str,
    mailbox: [Option<Message>; 10],
    mailbox_index: usize,
}

impl Process {
    pub fn new(name: &'static str) -> Self {
        let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
        Process {
            pid,
            name,
            mailbox: [None; 10],
            mailbox_index: 0,
        }
    }

    pub fn get_pid(&self) -> usize {
        self.pid
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn send_message(&self, receiver: &mut Process, msg_type: MessageType, data: &[u8]) -> bool {
        if receiver.mailbox_index >= receiver.mailbox.len() {
            println!("Process {} mailbox is full!", receiver.name);
            return false;
        }

        let mut message_data = [0u8; 32];
        let copy_len = core::cmp::min(data.len(), 32);
        message_data[..copy_len].copy_from_slice(&data[..copy_len]);

        let message = Message {
            sender_pid: self.pid,
            receiver_pid: receiver.pid,
            message_type: msg_type,
            data: message_data,
        };

        receiver.mailbox[receiver.mailbox_index] = Some(message);
        receiver.mailbox_index += 1;
        true
    }

    pub fn receive_message(&mut self) -> Option<Message> {
        if self.mailbox_index == 0 {
            return None;
        }

        let message = self.mailbox[0];

        // Shift all messages
        for i in 1..self.mailbox_index {
            self.mailbox[i-1] = self.mailbox[i];
        }

        self.mailbox[self.mailbox_index-1] = None;
        self.mailbox_index -= 1;

        message
    }
}

// Global process registry for demo purposes
lazy_static! {
    static ref KERNEL_PROCESS: Mutex<Process> = Mutex::new(Process::new("kernel"));
    static ref USER_PROCESS: Mutex<Process> = Mutex::new(Process::new("user_app"));
    static ref DEVICE_DRIVER: Mutex<Process> = Mutex::new(Process::new("device_driver"));
}

// Demo IPC functionality
pub fn demo_ipc() {
    // SECTION 1: IPC Introduction
    crate::vga_buffer::clear_screen();
    crate::vga_buffer::print_centered("IPC Demonstration", crate::vga_buffer::Color::LightCyan);
    crate::vga_buffer::print_centered("-------------------", crate::vga_buffer::Color::LightGray);
    println!();

    // Get process information
    let kernel_pid = KERNEL_PROCESS.lock().get_pid();
    let user_pid = USER_PROCESS.lock().get_pid();
    let driver_pid = DEVICE_DRIVER.lock().get_pid();

    println!("\nProcess Information:");
    println!("  - Kernel Process: PID {}", kernel_pid);
    println!("  - User Process: PID {}", user_pid);
    println!("  - Device Driver: PID {}", driver_pid);

    // Short pause
    crate::pause_execution(2);

    println!("\nSending messages between processes...");

    {
        let kernel = KERNEL_PROCESS.lock();
        let mut user = USER_PROCESS.lock();

        let data = b"Hello from kernel!";
        kernel.send_message(&mut user, MessageType::Command, data);
        println!("  - Kernel sent message to User Process");
    }

    {
        let mut kernel = KERNEL_PROCESS.lock();
        let user = USER_PROCESS.lock();

        let data = b"Hello from user app!";
        user.send_message(&mut kernel, MessageType::Response, data);
        println!("  - User Process sent message to Kernel");
    }

    {
        let mut user = USER_PROCESS.lock();
        let driver = DEVICE_DRIVER.lock();

        let data = b"Device status update";
        driver.send_message(&mut user, MessageType::Notification, data);
        println!("  - Device Driver sent message to User Process");
    }

    // Short pause
    crate::pause_execution(2);

    println!("\nProcessing received messages:");

    {
        let mut kernel = KERNEL_PROCESS.lock();
        if let Some(msg) = kernel.receive_message() {
            let sender_pid = msg.sender_pid;
            let msg_type = match msg.message_type {
                MessageType::Command => "Command",
                MessageType::Response => "Response",
                MessageType::Notification => "Notification",
            };

            // Convert bytes to string (only ASCII printable chars)
            let mut data_str = String::new();
            for &byte in msg.data.iter() {
                if byte == 0 {
                    break;
                }
                if byte >= 32 && byte <= 126 {
                    data_str.push(byte as char);
                }
            }

            println!("  - Kernel received {} from PID {}: \"{}\"",
                     msg_type, sender_pid, data_str);
        }
    }

    {
        let mut user = USER_PROCESS.lock();
        while let Some(msg) = user.receive_message() {
            let sender_pid = msg.sender_pid;
            let msg_type = match msg.message_type {
                MessageType::Command => "Command",
                MessageType::Response => "Response",
                MessageType::Notification => "Notification",
            };

            // Convert bytes to string (only ASCII printable chars)
            let mut data_str = String::new();
            for &byte in msg.data.iter() {
                if byte == 0 {
                    break;
                }
                if byte >= 32 && byte <= 126 {
                    data_str.push(byte as char);
                }
            }

            println!("  - User Process received {} from PID {}: \"{}\"",
                     msg_type, sender_pid, data_str);
        }
    }

    // Short pause
    crate::pause_execution(2);

    println!("\nIPC Demonstration completed successfully!");
    println!("-----------------------------------");

    // Final pause
    crate::pause_execution(2);
}
