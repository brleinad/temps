
export const getSavedLocations = () => {
  let savedLocations = localStorage.getItem('savedLocations');
  savedLocations = JSON.parse(localStorage.getItem('savedLocations'));
  if (!Array.isArray(savedLocations)) {
    savedLocations = [];
  }
  return savedLocations;
}
