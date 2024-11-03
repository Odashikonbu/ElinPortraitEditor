export interface config {
  language: string
  game_path: string
}

export const langs = () => {
  return [
    "ja",
    "en"
  ]
}

export const getXml = () => {
  return `
<?xml version="1.0" encoding="utf-8"?>
<Meta>
<title>Elin Custom Portrait</title>
<id>elin_customportrait</id>
<author>You</author>
<builtin>true</builtin>
<loadPriority>-100</loadPriority>
<version>0.22.1</version>
<description>Custom Portrait</description>
</Meta>
  `
}