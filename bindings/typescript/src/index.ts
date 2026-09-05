export * as validation from './validation';
export * from './generated';
export * from './pem';

export type Brand<T, TName extends string> = T & { readonly __brand: TName };

export type JSONValue =
  | string
  | number
  | boolean
  | null
  | readonly JSONValue[]
  | { readonly [key: string]: JSONValue };

export namespace Auth {
  export type JWT = Brand<string, 'Auth.JWT'>;
  export type Password = Brand<string, 'Auth.Password'>;
}

export namespace Contact {
  export type Email = Brand<string, 'Contact.Email'>;
  export type PhoneNumber = Brand<string, 'Contact.PhoneNumber'>;
}

export namespace Crypto {
  export type RSAPrivateKey = Brand<string, 'Crypto.RSAPrivateKey'>;
  export type RSAPublicKey = Brand<string, 'Crypto.RSAPublicKey'>;
  export type SHA256 = Brand<string, 'Crypto.SHA256'>;
}

export namespace Design {
  export type Color = Brand<string, 'Design.Color'>;
}

export namespace Embedding {
  export type Vector = Brand<readonly number[], 'Embedding.Vector'>;
}

export namespace File {
  export type SizeBytes = Brand<number, 'File.SizeBytes'>;
}

export namespace Finance {
  export type Money = Brand<number, 'Finance.Money'>;
}

export namespace Generic {
  export type Int64 = Brand<number, 'Generic.Int64'>;
  export type JSON = Brand<JSONValue, 'Generic.JSON'>;
  export type Probability = Brand<number, 'Generic.Probability'>;
  export type StringMap = Brand<Readonly<Record<string, string>>, 'Generic.StringMap'>;
}

export namespace Geo {
  export type Location = Brand<{ readonly lat: number; readonly lon: number }, 'Geo.Location'>;
}

export namespace Identity {
  export type Name = Brand<string, 'Identity.Name'>;
  export type Slug = Brand<string, 'Identity.Slug'>;
  export type UUID = Brand<string, 'Identity.UUID'>;
  export type UserID = Brand<string, 'Identity.UserID'>;
}

export namespace Localization {
  export type Locale = Brand<string, 'Localization.Locale'>;
}

export namespace Network {
  export type DnsLabel = Brand<string, 'Network.DnsLabel'>;
  export type DomainName = Brand<string, 'Network.DomainName'>;
  export type IpAddress = Brand<string, 'Network.IpAddress'>;
  export type Uri = Brand<string, 'Network.Uri'>;
  export type Url = Brand<string, 'Network.Url'>;
}

export namespace Temporal {
  export type CronExpression = Brand<string, 'Temporal.CronExpression'>;
  export type Date = Brand<string, 'Temporal.Date'>;
  export type DateTime = Brand<string, 'Temporal.DateTime'>;
  export type Days = Brand<number, 'Temporal.Days'>;
  export type Duration = Brand<string, 'Temporal.Duration'>;
  export type Hours = Brand<number, 'Temporal.Hours'>;
  export type Milliseconds = Brand<number, 'Temporal.Milliseconds'>;
  export type Minutes = Brand<number, 'Temporal.Minutes'>;
  export type Month = Brand<string, 'Temporal.Month'>;
  export type Quarter = Brand<string, 'Temporal.Quarter'>;
  export type QuarterYear = Brand<string, 'Temporal.QuarterYear'>;
  export type RecurrenceRule = Brand<string, 'Temporal.RecurrenceRule'>;
  export type Seconds = Brand<number, 'Temporal.Seconds'>;
  export type Time = Brand<string, 'Temporal.Time'>;
  export type TimeZone = Brand<string, 'Temporal.TimeZone'>;
  export type Year = Brand<string, 'Temporal.Year'>;
}

export namespace Text {
  export type Markdown = Brand<string, 'Text.Markdown'>;
  export type Sql = Brand<string, 'Text.Sql'>;
}

export type { ScalarBackend } from './backend';
export { loadBackend, napiBackend, wasmBackend } from './backend';
