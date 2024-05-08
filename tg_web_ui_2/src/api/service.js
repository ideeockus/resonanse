export const baseURL = "https://resonanse.ru/"


const appAPI = {
    async getEvents() {
        try {
            const res = await fetch(`${baseURL}api/events`).then(res => res.json())
            return res
        } catch (err) {
            console.log(err)
        }
    },
    async getEvent(id) {
        try {
            const res = await fetch(`${baseURL}api/events/${id}`).then(res => res.json())
            return res
        } catch (err) {
            console.log(err)
        }
    },
    async uploadImage(url) {
        try {
            const form = new FormData()
            form.append("file", url)
            const options = {
                method: "POST",
                "Content-Type": "multipart/form-data",
                body: form
            }
            const res = await fetch(`${baseURL}api/resources/upload-image`, options).then(res => res.json())
            return res
        } catch (err) {
            console.log(err)
        }
    },
    async getImage(id) {
        try {
            const res = await fetch(`${baseURL}api/resources/get-image/${id}`).then(res => res.blob())
            return res
        } catch (err) {
            console.log(err)
        }
    },
}

export default appAPI