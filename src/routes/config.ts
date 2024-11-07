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
  return  '<?xml version="1.0" encoding="utf-8"?>\n' + 
          '<Meta>\n' +
          '<title>Portrait</title>\n' +
          '<id>Elin Portrait</id>\n' +
          '<author>You</author>\n' +
          '<builtin>true</builtin>\n' +
          '<loadPriority>100</loadPriority>\n' +
          '<version>0.22.1</version>\n' +
          '<description>Portrait</description>\n' +
          '</Meta>'
}