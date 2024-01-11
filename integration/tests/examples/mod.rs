mod test_getting_started;

use assert_cmd::cargo::CommandCargoExt;
use assert_cmd::Command;
use std::process::Command as StdCommand;
use std::time::Duration;

pub(crate) struct IggyExampleTest<'a> {
    module: &'a str,
}

impl<'a> IggyExampleTest<'a> {
    pub(crate) fn new(module: &'a str) -> Self {
        Self { module }
    }

    pub(crate) async fn execute_test(&mut self) {
        let (producer_stdout, consumer_stdout) = self.spawn_executables().await;

        println!("PRODUCER: {}", producer_stdout);
        println!("CONSUMER: {}", consumer_stdout);
    }
}

impl<'a> IggyExampleTest<'a> {
    async fn spawn_executables(&mut self) -> (String, String) {
        let producer_binary = StdCommand::cargo_bin(format!("examples/{}-producer", self.module))
            .unwrap_or_else(|_| panic!("Failed to find {}-producer", self.module));
        let consumer_binary = StdCommand::cargo_bin(format!("examples/{}-consumer", self.module))
            .unwrap_or_else(|_| panic!("Failed to find {}-consumer", self.module));

        let mut producer_cmd = Command::new(producer_binary.get_program().to_str().unwrap());
        let mut consumer_cmd = Command::new(consumer_binary.get_program().to_str().unwrap());

        if let Ok(runner) = std::env::var("QEMU_RUNNER") {
            let mut producer_runner_command = Command::new(runner.clone());
            let mut consumer_runner_command = Command::new(runner);
            producer_runner_command.arg(producer_binary.get_program().to_str().unwrap());
            consumer_runner_command.arg(consumer_binary.get_program().to_str().unwrap());
            producer_cmd = producer_runner_command;
            consumer_cmd = consumer_runner_command
        };

        let producer_handle = tokio::spawn(async move {
            let producer_assert = producer_cmd.timeout(Duration::from_secs(1)).assert();
            let producer_output = producer_assert.get_output();
            String::from_utf8_lossy(&producer_output.stdout)
                .as_ref()
                .to_string()
        });
        let consumer_handle = tokio::spawn(async move {
            let consumer_assert = consumer_cmd.timeout(Duration::from_secs(1)).assert();
            let consumer_output = consumer_assert.get_output();
            String::from_utf8_lossy(&consumer_output.stdout)
                .as_ref()
                .to_string()
        });
        (
            producer_handle.await.unwrap(),
            consumer_handle.await.unwrap(),
        )
    }
}
