use std::path::Path;
use reqwest;

const RPC_QUEUE_RECOMMENDATION_BY_USER: &str = "recommendations.requests.by_user";
const RPC_QUEUE_SET_USER_DESCRIPTION: &str = "resonanse_api.requests.set_user_description";

pub fn rpc_get_recommendation_by_user(user_id: i64) {
    // todo rpc get recommendation
}

pub fn rpc_set_user_description(user_id: i64, description: String) {
    // todo rpc set description
}



// todo check this carefully
async fn download_image(url: &str, path: &Path) -> Result<(), Error> {
    // Отправляем GET запрос для скачивания изображения
    let response = reqwest::get(url).await?;

    // Проверяем статус ответа
    if response.status().is_success() {
        // Открываем файл для записи
        let mut file = OpenOptions::new().create(true).write(true).open(path).await?;

        // Получаем поток данных из ответа
        let mut content = response.bytes_stream();

        // Записываем данные в файл
        while let Some(chunk) = content.next().await {
            let chunk = chunk?;
            tokio::io::copy(&mut chunk.as_ref(), &mut file).await?;
        }

        Ok(())
    } else {
        Err(reqwest::Error::new(reqwest::StatusCode::from_u16(response.status().as_u16()).unwrap(), "Failed to download image"))
    }
}