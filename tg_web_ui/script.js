// script.js
document.addEventListener('DOMContentLoaded', function () {
    fetchEvents();
});

function fetchEvents() {
    const eventsEndpoint = 'https://resonanse.ru/api/events'; // Актуальный URL API

    fetch(eventsEndpoint)
        .then(response => {
            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status}`);
            }
            return response.json();
        })
        .then(events => {
            const eventsContainer = document.getElementById('events-container');
            events.forEach(event => {
                // Создание элементов для каждого события
                const eventWrapper = document.createElement('div');
                eventWrapper.className = 'gjs-grid-column';

                // Заполнение содержимого события
                eventWrapper.innerHTML = `
                    <div class="gjs-grid-row event-wrapper-row">
                        <div class="gjs-grid-column event-wrapper-left-column">
                            <h2 class="event-title">${event.title}</h2>
                            <p class="event-description">${event.description}</p>
                        </div>
                        <div class="gjs-grid-column">
                            <img src="https://resonanse.ru//api/resources/get-image/${event.poster_image_link}" alt="Event Image"/>
                        </div>
                    </div>
                `;

                eventsContainer.appendChild(eventWrapper);
            });
        })
        .catch(error => {
            console.error('Error fetching events:', error);
            // Обработка ошибок
        });
}

document.getElementById('create-event-btn').addEventListener('click', function() {
    document.getElementById('event-modal').style.display = 'block';
});

document.querySelector('.close-btn').addEventListener('click', function() {
    document.getElementById('event-modal').style.display = 'none';
});

document.getElementById('create-event-form').addEventListener('submit', function(e) {
    e.preventDefault();

    const eventData = {
        title: document.getElementById('event-title-input').value,
        description: document.getElementById('event-description-input').value,
        short_description: document.getElementById('event-short-description-input').value,
        category: document.getElementById('event-category-input').value,
        location: document.getElementById('event-location-input').value,
        start_date: document.getElementById('event-start-date-input').value,
        end_date: document.getElementById('event-end-date-input').value,
        online: document.getElementById('event-online-input').checked,
        attendance_confirmation_days_before: parseInt(document.getElementById('event-attendance-confirmation-days-input').value, 10),
        chat_link: document.getElementById('event-chat-link-input').value,
        organizer_id: parseInt(document.getElementById('event-organizer-id-input').value, 10),
        community_id: parseInt(document.getElementById('event-community-id-input').value, 10),
        poster_image_link: document.getElementById('event-poster-image-link-input').value,
        paid: document.getElementById('event-paid-input').checked
    };

    fetch('https://resonanse.ru/api/events', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(eventData)
    })
    .then(response => {
        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }
        return response.json();
    })
    .then(data => {
        console.log(data);
        document.getElementById('event-modal').style.display = 'none';
        document.getElementById('create-event-form').reset();
    })
    .catch(error => {
        console.error('Error creating event:', error);
    });
});
