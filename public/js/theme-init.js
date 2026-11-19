(function () {
  try {
    var saved = localStorage.getItem("mallen_theme_mode")
      || localStorage.getItem("w16_theme_mode")
      || localStorage.getItem("mallen_theme");
    if (saved === "light") {
      document.documentElement.classList.add("light-mode");
    }
  } catch (_) {}
})();
