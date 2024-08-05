document.addEventListener('scroll', () => {
    const navbar = document.querySelector('.navbar');
    if (window.scrollY > 50) { // Adjust scroll position to trigger the change
        navbar.classList.add('scrolled');
    } else {
        navbar.classList.remove('scrolled');
    }
});