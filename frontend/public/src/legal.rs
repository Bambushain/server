use crate::utils::banner_page;
use actix_web::{get, Responder};
use bamboo_common::core::entities::get_dependencies;
use chrono::Datelike;
use maud::html;

#[get("/legal/legal-notice")]
pub async fn legal_notice() -> impl Responder {
    banner_page(
        "Impressum",
        html! {
            h2 {
                "Angaben gemäß § 5 TMG"
            }
            address {
                "Imanuel Ulbricht"
                br {}
                "Dingworthstr. 15"
                br {}
                "31137 Hildesheim"
            }
            h3 {
                "Kontakt"
            }
            p {
                "E-Mail: "
                a href="mailto:panda.helferlein@bambushain.app" {
                    "panda.helferlein@bambushain.app"
                }
            }
            h3 {
                "Haftung für Inhalte"
            }
            p {
                "Als Diensteanbieter sind wir gemäß § 7 Abs.1 TMG für eigene Inhalte auf diesen Seiten nach den allgemeinen Gesetzen verantwortlich. Nach §§ 8 bis 10 TMG sind wir als Diensteanbieter jedoch nicht verpflichtet, übermittelte oder gespeicherte fremde Informationen zu überwachen oder nach Umständen zu forschen, die auf eine rechtswidrige Tätigkeit hinweisen."
            }
            p {
                "Verpflichtungen zur Entfernung oder Sperrung der Nutzung von Informationen nach den allgemeinen Gesetzen bleiben hiervon unberührt. Eine diesbezügliche Haftung ist jedoch erst ab dem Zeitpunkt der Kenntnis einer konkreten Rechtsverletzung möglich. Bei Bekanntwerden von entsprechenden Rechtsverletzungen werden wir diese Inhalte umgehend entfernen."
            }
            p {
                "Quelle: e-recht24.de"
            }
        },
    )
}

#[get("/legal/licenses")]
pub async fn licenses() -> impl Responder {
    let dependencies = get_dependencies();
    let year = chrono::Local::now().year();
    banner_page(
        "Bambushains Lizenz",
        html! {
            p {
                "Bambushain verwendet diverse Bilder, Schriften und Softwarekomponenten, hier findest du eine Liste mit den Lizenzen"
            }
            h2 {
                "Bambushains Lizenz"
            }
            p {
                "Bambushain ist unter der Open Source MIT Lizenz veröffentlicht, den Code kannst du dir hier anschauen:"
                a href="https://github.com/Bambushain" {
                    "Github"
                }
            }
            details {
                summary {
                    "Lizenztext"
                }
                pre {
                    (format!("MIT License
Copyright (c) {year} Imanuel Ulbricht, Christina Ruebsam and Hans-Jürgen Wandschneider
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."))
                }
            }
            h2 {
                "Bildlizenzen"
            }
            h3 {
                "Hintergrundbild"
            }
            p {
                "Bambushain hat ein Hintergrundbild von "
                a href="https://unsplash.com" {
                    "Unsplash"
                }
                ". Das Bild steht unter der Unsplash Lizenz, die wir unten verlinkt und kopiert haben. Das Bild haben wir auch verlinkt und noch einmal ohne Filter gezeigt."
            }
            h4 {
                "Bild"
            }
            figure."bamboo-preview is--license" {
                img."bamboo-preview__image is--license" src="/static/background-pandas.webp" {}
                figcaption."bamboo-preview__caption is--license" {
                    a href="https://unsplash.com/de/fotos/ein-holzsteg-mit-pflanzen-und-baumen-l3lxbhkTApM" {
                        "Bildherkunft"
                    }
                }
            }
            h4 {
                "Lizenz"
            }
            p {
                "Unsplash gewährt Ihnen eine unwiderrufliche, nicht-exklusive, weltweite Urheberrechtslizenz, um Fotos von Unsplash kostenlos herunterzuladen, zu kopieren, zu verändern, zu verbreiten, aufzuführen und zu nutzen, auch für kommerzielle Zwecke, ohne Genehmigung oder Nennung des Fotografen oder Unsplash. Diese Lizenz umfasst nicht das Recht, Fotos von Unsplash zu kompilieren, um einen ähnlichen oder konkurrierenden Service zu replizieren."
            }
            p {
                "Du findest die Unsplash Lizenz hier:"
                a href="https://unsplash.com/de/lizenz" {
                    "Unsplash Lizenz"
                }
            }
            h3 {
                "Login Hintergrundbild"
            }
            p {
                "Bambushain hat ein Login Hintergrundbild von "
                a href="https://unsplash.com" {
                    "Unsplash"
                }
                ". Das Bild steht unter der Unsplash Lizenz, die wir unten verlinkt und kopiert haben. Das Bild haben wir auch verlinkt und noch einmal ohne Filter gezeigt."
            }
            h4 {
                "Bild"
            }
            figure."bamboo-preview is--license" {
                img."bamboo-preview__image is--license" src="/static/background.webp" {}
                figcaption."bamboo-preview__caption is--license" {
                    a href="https://unsplash.com/de/fotos/haus-zwischen-wald-F-6rwMp1M-Q" {
                        "Bildherkunft"
                    }
                }
            }
            h4 {
                "Lizenz"
            }
            p {
                "Unsplash gewährt Ihnen eine unwiderrufliche, nicht-exklusive, weltweite Urheberrechtslizenz, um Fotos von Unsplash kostenlos herunterzuladen, zu kopieren, zu verändern, zu verbreiten, aufzuführen und zu nutzen, auch für kommerzielle Zwecke, ohne Genehmigung oder Nennung des Fotografen oder Unsplash. Diese Lizenz umfasst nicht das Recht, Fotos von Unsplash zu kompilieren, um einen ähnlichen oder konkurrierenden Service zu replizieren."
            }
            p {
                "Du findest die Unsplash Lizenz hier:"
                a href="https://unsplash.com/de/lizenz" {
                    "Unsplash Lizenz"
                }
            }
            h3 {
                "Logo"
            }
            p {
                "Das aktuelle Bambushain Logo ist von"
                a href="https://www.svgrepo.com/" {
                    "SVG Repo"
                }
                ". Das Bild steht unter der CC0 Lizenz, die wir unten verlinkt und kopiert haben. Das Bild haben wir auch verlinkt und noch einmal ohne Filter gezeigt."
            }
            h4 {
                "Bild"
            }
            figure."bamboo-preview is--license" {
                img."bamboo-preview__image is--license is--small" src="/public/assets/favicon.svg" {}
                figcaption."bamboo-preview__caption is--license" {
                    a href="https://www.svgrepo.com/svg/220234/bamboo" {
                        "Bildherkunft"
                    }
                }
            }
            details {
                summary {
                    "Lizenztext"
                }
                h4 {
                    "Public Domain"
                }
                p {
                    em {
                        "(As visual work) This license also might be referred as No copyright or CC0 1.0 Universal PD Dedication on our website."
                    }
                }
                p {
                    "The person who associated a work with this deed has dedicated the work to the public domain by waiving all of his or her rights to the work worldwide under copyright law, including all related and neighboring rights, to the extent allowed by law."
                }
                p {
                    "Or, the work consists of simple geometry and is not ineligable for copyright due to Threshold of originality (this threshold might vary depending on different country laws). For an example \"A stick figure, where the head is represented by a circle and other parts represented by straight lines\" is not copyrightable or falls into public domain."
                }
                p {
                    "You are free:"
                }
                ul {
                    li {
                        strong {
                            "to share"
                        }
                        "– to copy, distribute and transmit the work"
                    }
                    li {
                        strong {
                            "to remix"
                        }
                        "– to adapt the work"
                    }
                }
                p {
                    "Under the following terms:"
                }
                ul {
                    li {
                        strong {
                            "attribution"
                        }
                        "– there is no author or author waived their right, no attribution"
                    }
                    li {
                        strong {
                            "share alike"
                        }
                        "– If you remix, transform, or build upon the material, you can distribute your work under any license."
                    }
                }
                p {
                    "Unless in the single pages of icons indexed on this website indicates differently, simple icons distributed on this website is subject to public domain or open source."
                }
                p {
                    "To learn more about this license,"
                    a href="https://creativecommons.org/publicdomain/zero/1.0/deed.en" {
                        "check out this page"
                    }
                    "."
                }
                p {
                    "Du findest die CC0 Lizenz hier:"
                    a href="https://www.svgrepo.com/page/licensing/id=CC0" {
                        "CC0 Lizenz"
                    }
                }
            }
            h2 {
                "Schriftlizenzen"
            }
            h3 {
                "Albert Sans"
            }
            p {
                "Die Standardschrift von Bambushain ist"
                a href="https://github.com/usted/Albert-Sans" {
                    "Albert Sans"
                }
                ". Albert Sans ist unter der SIL Open Font License lizensiert. Details dazu unten."
            }
            details {
                summary {
                    "Lizenztext"
                }
                pre {
                    "Copyright 2021 The Albert Sans Project Authors (https://github.com/usted/Albert-Sans)
This Font Software is licensed under the SIL Open Font License, Version 1.1.
This license is copied below, and is also available with a FAQ at:
https://scripts.sil.org/OFL
-----------------------------------------------------------
SIL OPEN FONT LICENSE Version 1.1 - 26 February 2007
-----------------------------------------------------------
PREAMBLE
The goals of the Open Font License (OFL) are to stimulate worldwide
development of collaborative font projects, to support the font creation
efforts of academic and linguistic communities, and to provide a free and
open framework in which fonts may be shared and improved in partnership
with others.
The OFL allows the licensed fonts to be used, studied, modified and
redistributed freely as long as they are not sold by themselves. The
fonts, including any derivative works, can be bundled, embedded,
redistributed and/or sold with any software provided that any reserved
names are not used by derivative works. The fonts and derivatives,
however, cannot be released under any other type of license. The
requirement for fonts to remain under this license does not apply
to any document created using the fonts or their derivatives.
DEFINITIONS
\"Font Software\" refers to the set of files released by the Copyright
Holder(s) under this license and clearly marked as such. This may
include source files, build scripts and documentation.
\"Reserved Font Name\" refers to any names specified as such after the
copyright statement(s).
\"Original Version\" refers to the collection of Font Software components as
distributed by the Copyright Holder(s).
\"Modified Version\" refers to any derivative made by adding to, deleting,
or substituting -- in part or in whole -- any of the components of the
Original Version, by changing formats or by porting the Font Software to a
new environment.
\"Author\" refers to any designer, engineer, programmer, technical
writer or other person who contributed to the Font Software.
PERMISSION & CONDITIONS
Permission is hereby granted, free of charge, to any person obtaining
a copy of the Font Software, to use, study, copy, merge, embed, modify,
redistribute, and sell modified and unmodified copies of the Font
Software, subject to the following conditions:
1) Neither the Font Software nor any of its individual components,
in Original or Modified Versions, may be sold by itself.
2) Original or Modified Versions of the Font Software may be bundled,
redistributed and/or sold with any software, provided that each copy
contains the above copyright notice and this license. These can be
included either as stand-alone text files, human-readable headers or
in the appropriate machine-readable metadata fields within text or
binary files as long as those fields can be easily viewed by the user.
3) No Modified Version of the Font Software may use the Reserved Font
Name(s) unless explicit written permission is granted by the corresponding
Copyright Holder. This restriction only applies to the primary font name as
presented to the users.
4) The name(s) of the Copyright Holder(s) or the Author(s) of the Font
Software shall not be used to promote, endorse or advertise any
Modified Version, except to acknowledge the contribution(s) of the
Copyright Holder(s) and the Author(s) or with their explicit written
permission.
5) The Font Software, modified or unmodified, in part or in whole,
must be distributed entirely under this license, and must not be
distributed under any other license. The requirement for fonts to
remain under this license does not apply to any document created
using the Font Software.
TERMINATION
This license becomes null and void if any of the above conditions are
not met.
DISCLAIMER
THE FONT SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT
OF COPYRIGHT, PATENT, TRADEMARK, OR OTHER RIGHT. IN NO EVENT SHALL THE
COPYRIGHT HOLDER BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
INCLUDING ANY GENERAL, SPECIAL, INDIRECT, INCIDENTAL, OR CONSEQUENTIAL
DAMAGES, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF THE USE OR INABILITY TO USE THE FONT SOFTWARE OR FROM
OTHER DEALINGS IN THE FONT SOFTWARE."
                }
            }
            h3 {
                "Urbanist"
            }
            p {
                "Für Menüs, Überschriften und ähnliches benutzt Bambushain"
                a href="https://github.com/coreyhu/Urbanist" {
                    "Urbanist"
                }
                ". Urbanist ist unter der SIL Open Font License lizensiert. Details dazu unten."
            }
            details {
                summary {
                    "Lizenztext"
                }
                pre {
                    "Copyright 2024 The Urbanist Project Authors (https://github.com/coreyhu/Urbanist)
This Font Software is licensed under the SIL Open Font License, Version 1.1.
This license is copied below, and is also available with a FAQ at:
https://scripts.sil.org/OFL
-----------------------------------------------------------
SIL OPEN FONT LICENSE Version 1.1 - 26 February 2007
-----------------------------------------------------------
PREAMBLE
The goals of the Open Font License (OFL) are to stimulate worldwide
development of collaborative font projects, to support the font creation
efforts of academic and linguistic communities, and to provide a free and
open framework in which fonts may be shared and improved in partnership
with others.
The OFL allows the licensed fonts to be used, studied, modified and
redistributed freely as long as they are not sold by themselves. The
fonts, including any derivative works, can be bundled, embedded,
redistributed and/or sold with any software provided that any reserved
names are not used by derivative works. The fonts and derivatives,
however, cannot be released under any other type of license. The
requirement for fonts to remain under this license does not apply
to any document created using the fonts or their derivatives.
DEFINITIONS
\"Font Software\" refers to the set of files released by the Copyright
Holder(s) under this license and clearly marked as such. This may
include source files, build scripts and documentation.
\"Reserved Font Name\" refers to any names specified as such after the
copyright statement(s).
\"Original Version\" refers to the collection of Font Software components as
distributed by the Copyright Holder(s).
\"Modified Version\" refers to any derivative made by adding to, deleting,
or substituting -- in part or in whole -- any of the components of the
Original Version, by changing formats or by porting the Font Software to a
new environment.
\"Author\" refers to any designer, engineer, programmer, technical
writer or other person who contributed to the Font Software.
PERMISSION & CONDITIONS
Permission is hereby granted, free of charge, to any person obtaining
a copy of the Font Software, to use, study, copy, merge, embed, modify,
redistribute, and sell modified and unmodified copies of the Font
Software, subject to the following conditions:
1) Neither the Font Software nor any of its individual components,
in Original or Modified Versions, may be sold by itself.
2) Original or Modified Versions of the Font Software may be bundled,
redistributed and/or sold with any software, provided that each copy
contains the above copyright notice and this license. These can be
included either as stand-alone text files, human-readable headers or
in the appropriate machine-readable metadata fields within text or
binary files as long as those fields can be easily viewed by the user.
3) No Modified Version of the Font Software may use the Reserved Font
Name(s) unless explicit written permission is granted by the corresponding
Copyright Holder. This restriction only applies to the primary font name as
presented to the users.
4) The name(s) of the Copyright Holder(s) or the Author(s) of the Font
Software shall not be used to promote, endorse or advertise any
Modified Version, except to acknowledge the contribution(s) of the
Copyright Holder(s) and the Author(s) or with their explicit written
permission.
5) The Font Software, modified or unmodified, in part or in whole,
must be distributed entirely under this license, and must not be
distributed under any other license. The requirement for fonts to
remain under this license does not apply to any document created
using the Font Software.
TERMINATION
This license becomes null and void if any of the above conditions are
not met.
DISCLAIMER
THE FONT SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT
OF COPYRIGHT, PATENT, TRADEMARK, OR OTHER RIGHT. IN NO EVENT SHALL THE
COPYRIGHT HOLDER BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
INCLUDING ANY GENERAL, SPECIAL, INDIRECT, INCIDENTAL, OR CONSEQUENTIAL
DAMAGES, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF THE USE OR INABILITY TO USE THE FONT SOFTWARE OR FROM
OTHER DEALINGS IN THE FONT SOFTWARE."
                }
            }
            h2 {
                "Softwarelizenzen"
            }
            @for dependency in &dependencies {
                h3 {
                    (dependency.name)
                }
                dl {
                    dt {
                        "Beschreibung"
                    }
                    dd {
                        (dependency.description)
                    }
                    dt {
                        "Autoren"
                    }
                    dd {
                        (dependency.authors)
                    }
                    dt {
                        "Quelle"
                    }
                    dd {
                        a href=(dependency.repository) {
                            (dependency.repository)
                        }
                    }
                    dt {
                        "Lizenz"
                    }
                    dd {
                        (dependency.license)
                    }
                }
            }
        },
    )
}

#[get("/legal/privacy")]
pub async fn privacy() -> impl Responder {
    banner_page(
        "Datenschutzerklärung",
        html! {
            p {
                "Mit dieser Datenschutzerklärung möchten wir Sie über Art, Umfang und Zweck der Verarbeitung von personenbezogenen Daten (im Folgenden auch nur als \"Daten\" bezeichnet) aufklären. Personenbezogene Daten sind alle Daten, die einen persönlichen Bezug zu Ihnen aufweisen, z. B. Name, Adresse, E-Mail-Adresse oder Ihr Nutzerverhalten. Die Datenschutzerklärung gilt für alle von uns vorgenommene Daten-Verarbeitungsvorgänge sowohl im Rahmen unserer Kerntätigkeit als auch für die von uns vorgehaltenen Online-Medien."
            }
            h2 {
                "Wer bei uns für die Datenverarbeitung verantwortlich ist"
            }
            p {
                "Verantwortlich für die Datenverarbeitung ist:"
            }
            address {
                "Imanuel Ulbricht"
                br ;
                "Dingworthstr. 15"
                br ;
                "31137 Hildesheim"
                br ;
                "Telefon: +49 1525 5709066"
                br ;
                "E-Mail: "
                a href="mailto:panda.helferlein@bambushain.app" {
                    "panda.helferlein@bambushain.app"
                }
            }
            p {
                "Nach der DSGVO stehen Ihnen die nachfolgend aufgeführten Rechte zu, die Sie jederzeit bei dem in Ziffer 1. dieser Datenschutzerklärung genannten Verantwortlichen geltend machen können:"
            }
            dl {
                dt {
                    "Recht auf Auskunft"
                }
                dd {
                    "Sie haben das Recht, von uns Auskunft darüber zu verlangen, ob und welche Daten wir von Ihnen verarbeiten."
                }
                dt {
                    "Recht auf Berichtigung"
                }
                dd {
                    "Sie haben das Recht, die Berichtigung unrichtiger oder Vervollständigung unvollständiger Daten zu verlangen."
                }
                dt {
                    "Recht auf Löschung"
                }
                dd {
                    "Sie haben das Recht, die Löschung Ihrer Daten zu verlangen."
                }
                dt {
                    "Recht auf Einschränkung"
                }
                dd {
                    "Sie haben in bestimmten Fällen das Recht zu verlangen, dass wir Ihre Daten nur noch eingeschränkt bearbeiten."
                }
                dt {
                    "Recht auf Datenübertragbarkeit"
                }
                dd {
                    "Sie haben das Recht zu verlangen, dass wir Ihnen oder einem anderen Verantwortlichen Ihre Daten in einem strukturierten, gängigen und maschinenlesebaren Format übermitteln."
                }
                dt {
                    "Beschwerderecht"
                }
                dd {
                    "Sie haben das Recht, sich bei einer Aufsichtsbehörde zu beschweren. Zuständig ist die Aufsichtsbehörde Ihres üblichen Aufenthaltsortes, Ihres Arbeitsplatzes oder unseres Firmensitzes."
                }
            }
            h3 {
                "Widerrufsrecht"
            }
            p {
                "Sie haben das Recht, die von Ihnen erteilte Einwilligung zur Datenverarbeitung jederzeit zu widerrufen."
            }
            h3 {
                "Widerspruchsrecht"
            }
            p {
                "Sie haben das Recht, jederzeit gegen die Verarbeitung Ihrer Daten, die wir auf unser berechtigtes Interesse nach Art. 6 Abs. 1 lit. f DSGVO stützen, Widerspruch einzulegen. Sofern Sie von Ihrem Widerspruchsrecht Gebrauch machen, bitten wir Sie um die Darlegung der Gründe. Wir werden Ihre personenbezogenen Daten dann nicht mehr verarbeiten, es sei denn, wir können Ihnen gegenüber nachweisen, dass zwingende schutzwürdige Gründe an der Datenverarbeitung Ihre Interessen und Rechte überwiegen."
            }
            p {
                "Unabhängig vom vorstehend Gesagten, haben Sie das jederzeitige Recht, der Verarbeitung Ihrer personenbezogenen Daten für Zwecke der Werbung und Datenanalyse zu widersprechen."
            }
            p {
                "Ihren Widerspruch richten Sie bitte an die oben angegebene Kontaktadresse des Verantwortlichen."
            }
            h2 {
                "Wann löschen wir Ihre Daten?"
            }
            p {
                "Wir löschen Ihre Daten dann, wenn wir diese nicht mehr brauchen oder Sie uns dies vorgeben. Das bedeutet, dass - sofern sich aus den einzelnen Datenschutzhinweisen dieser Datenschutzerklärung nichts anderes ergibt - wir Ihre Daten löschen,"
            }
            ul {
                li {
                    "wenn der Zweck der Datenverarbeitung weggefallen ist und damit die jeweilige in den einzelnen Datenschutzhinweisen genannte Rechtsgrundlage nicht mehr besteht, also bspw."
                    ul {
                        li {
                            "nach Beendigung der zwischen uns bestehenden vertraglichen oder mitgliedschaftlichen Beziehungen (Art. 6 Abs. 1 lit. a DSGVO) oder"
                        }
                        li {
                            "nach Wegfall unseres berechtigten Interesses an der weiteren Verarbeitung oder Speicherung Ihrer Daten (Art. 6 Abs. 1 lit. f DSGVO),"
                        }
                    }
                }
                li {
                    "wenn Sie von Ihrem Widerrufsrecht Gebrauch machen und keine anderweitige gesetzliche Rechtsgrundlage für die Verarbeitung im Sinne von Art. 6 Abs. 1 lit. b-f DSGVO eingreift,"
                }
                li {
                    "wenn Sie vom Ihrem Widerspruchsrecht Gebrauch machen und der Löschung keine zwingenden schutzwürdigen Gründe entgegenstehen."
                }
            }
            p {
                "Sofern wir (bestimmte Teile) Ihre(r) Daten jedoch noch für andere Zwecke vorhalten müssen, weil dies etwa steuerliche Aufbewahrungsfristen (in der Regel 6 Jahre für Geschäftskorrespondenz bzw. 10 Jahre für Buchungsbelege) oder die Geltendmachung, Ausübung oder Verteidigung von Rechtsansprüchen aus vertraglichen Beziehungen (bis zu vier Jahren) erforderlich machen oder die Daten zum Schutz der Rechte einer anderen natürlichen oder juristischen Person gebraucht werden, löschen wir (den Teil) Ihre(r) Daten erst nach Ablauf dieser Fristen. Bis zum Ablauf dieser Fristen beschränken wir die Verarbeitung dieser Daten jedoch auf diese Zwecke (Erfüllung der Aufbewahrungspflichten)."
            }
            p {
                "Unsere Internetseite nutzt Cookies. Bei Cookies handelt es sich um kleine Textdateien, bestehend aus einer Reihe von Zahlen und Buchstaben, die auf dem von Ihnen genutzten Endgerät abgelegt und gespeichert werden. Cookies dienen vorrangig dazu, Informationen zwischen dem von Ihnen genutzten Endgerät und unserer Webseite auszutauschen. Hierzu gehören u. a. die Spracheinstellungen auf einer Webseite, der Login-Status oder die Stelle, an der ein Video geschaut wurde."
            }
            p {
                "Beim Besuch unserer Webseiten wird ein permanenter Cookie eingesetzt, dieser Cookie speichert lediglich die Information für den Login und wird für den Kalender benötigt."
            }
            h2 {
                "Kontaktaufnahme"
            }
            p {
                "Soweit Sie uns über E-Mail, Telefon, Post, unser Kontaktformular oder sonstwie ansprechen und uns hierbei personenbezogene Daten wie Ihren Namen, Ihre Telefonnummer oder Ihre E-Mail-Adresse zur Verfügung stellen oder weitere Angaben zur Ihrer Person oder Ihrem Anliegen machen, verarbeiten wir diese Daten zur Beantwortung Ihrer Anfrage im Rahmen des zwischen uns bestehenden vorvertraglichen oder vertraglichen Beziehungen."
            }
            dl {
                dt {
                    "Betroffene Daten"
                }
                dd {
                    ul {
                        li {
                            "Bestandsdaten (bspw. Namen, Adressen)"
                        }
                        li {
                            "Kontakdaten (bspw. E-Mail-Adresse, Telefonnummer, Postanschrift)"
                        }
                        li {
                            "Inhaltsdaten (Texte, Fotos, Videos)"
                        }
                        li {
                            "Vertragsdaten (bspw. Vertragsgegenstand, Vertragsdauer)"
                        }
                    }
                }
                dt {
                    "Betroffene Personen"
                }
                dd {
                    "Interessenten, Kunden, Geschäfts- und Vertragspartner"
                }
                dt {
                    "Verarbeitungszweck"
                }
                dd {
                    "Kommunikation sowie Beantwortung von Kontaktanfragen, Büro und Organisationsverfahren"
                }
                dt {
                    "Rechtsgrundlage"
                }
                dd {
                    "Vertragserfüllung und vorvertragliche Anfragen, Art. 6 Abs. 1 lit. b DSGVO, berechtigtes Interesse, Art. 6 Abs. 1 lit. f DSGVO"
                }
            }
            h2 {
                "Sicherheitsmaßnahmen"
            }
            p {
                "Wir treffen im Übrigen technische und organisatorische Sicherheitsmaßnahmen nach dem Stand der Technik, um die Vorschriften der Datenschutzgesetze einzuhalten und Ihre Daten gegen zufällige oder vorsätzliche Manipulationen, teilweisen oder vollständigen Verlust, Zerstörung oder gegen den unbefugten Zugriff Dritter zu schützen."
            }
            h2 {
                "Aktualität und Änderung dieser Datenschutzerklärung"
            }
            p {
                "Diese Datenschutzerklärung ist aktuell gültig und hat den Stand Dezember 2023. Aufgrund geänderter gesetzlicher bzw. behördlicher Vorgaben kann es notwendig werden, diese Datenschutzerklärung anzupassen."
            }
            p {
                "Diese Datenschutzerklärung wurde mit Hilfe des Datenschutz-Generators von SOS Recht erstellt. SOS Recht ist ein Angebot der Mueller.legal Rechtsanwälte Partnerschaft mit Sitz in Berlin."
            }
        },
    )
}
