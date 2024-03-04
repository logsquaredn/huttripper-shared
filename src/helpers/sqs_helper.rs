use aws_sdk_sqs::{error::SdkError, operation::{get_queue_url::GetQueueUrlError, receive_message::ReceiveMessageError, send_message::SendMessageError}, types::Message, Client};

pub struct SQSHelper {
    pub sqs_client: Client,
    pub queue_url: String
}

pub async fn create_sqs_helper(aws_config: &aws_config::SdkConfig, queue_name: &str) -> Result<SQSHelper, SdkError<GetQueueUrlError>> {
    let sqs_client = aws_sdk_sqs::Client::new(aws_config);

    let q_url_output = sqs_client
        .get_queue_url()
        .queue_name(queue_name)
        .send()
        .await?;

    let queue_url = q_url_output.queue_url.expect(&format!("failed to get queue url for queue name: {}", queue_name));
    Ok(SQSHelper {
        sqs_client,
        queue_url
    })
}

impl SQSHelper {

    pub async fn send_message(&self, body: &str) -> Result<(), SdkError<SendMessageError>> {
        self.sqs_client
            .send_message()
            .queue_url(&self.queue_url)
            .message_body(body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn receive_messages(&self) -> Result<Vec<Message>, SdkError<ReceiveMessageError>> {
        let rec_msg_output = self.sqs_client
            .receive_message()
            .queue_url(&self.queue_url)
            .send()
            .await?;

        Ok(rec_msg_output.messages.unwrap_or(Vec::new()))
    }
}