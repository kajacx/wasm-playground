// export const cargoCompImports = {
//   "import-point": (point) => {
//     point.x = point.x + 100;
//     return point;
//   },
//   importPoint(point) {
//     point.x = point.x + 1000;
//     return point;
//   },
// };

export default function (point) {
  point.x = point.x + 100;
  return point;
}
