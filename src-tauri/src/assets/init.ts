function init() {
  setTimeout(() => {
    const text = document.querySelector(".text-xs.text-center");
    if (text) text.remove();
  }, 200);
}

if (document.readyState === "complete" || document.readyState === "interactive") {
  init();
} else {
  document.addEventListener("DOMContentLoaded", init);
}
