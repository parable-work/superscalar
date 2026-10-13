// A site's use of the package, which scripts/web_smoke.sh bundles with
// `bun build --target=browser` and no plugin, then runs in Node; index.html
// beside it loads the same bundle in a browser. It imports the entries a
// generated SDK imports, validates through the WASM backend and throws on a
// wrong answer.
import { parseContactEmail, validateIdentityUUID } from "superscalar";
import { parseGeoLocationStrict } from "superscalar/scalars";
import { hasErrors, newValidationErrors } from "superscalar/validation";

const result = {
  email: parseContactEmail(" Ada.Lovelace@Example.COM "),
  rejectsEmail: parseContactEmail("not-an-email") === null,
  uuid: validateIdentityUUID("0f8fad5b-d9cb-469f-a165-70867728950e")[0],
  rejectsUUID: validateIdentityUUID("not-a-uuid")[0] === false,
  location: parseGeoLocationStrict('{"lon": 2, "lat": 1}'),
  noErrors: !hasErrors(newValidationErrors()),
};
const want = JSON.stringify({
  email: "ada.lovelace@example.com",
  rejectsEmail: true,
  uuid: true,
  rejectsUUID: true,
  location: { lat: 1, lon: 2 },
  noErrors: true,
});
if (JSON.stringify(result) !== want) {
  throw new Error(`superscalar site bundle: got ${JSON.stringify(result)}, want ${want}`);
}
globalThis.superscalarSmoke = result;
console.log("superscalar site bundle: ok");
