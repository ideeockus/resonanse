// script.js
document.addEventListener('DOMContentLoaded', function () {
    fetchEvents();
});

function fetchEvents() {
    // The endpoint to fetch events; replace with your actual endpoint
    const eventsEndpoint = 'https://resonanse.ru/api/events';

    fetch(eventsEndpoint)
        .then(response => {
            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status}`);
            }
            return response.json();
        })
        .then(events => {
            const eventsContainer = document.getElementById('events');
            events.forEach(event => {
                const eventElement = document.createElement('div');
                eventElement.className = 'event-item';
                eventElement.innerHTML = `
                    <h3 class="event-title">${event.title}</h3>
                    <p class="event-date">${new Date(event.start_date).toLocaleString()}</p>
                    <p>${event.description}</p>
                `;
                eventsContainer.appendChild(eventElement);
            });
        })
        .catch(error => {
            console.error('Error fetching events:', error);
            // Handle errors, such as by displaying an error message on the page
        });
}
