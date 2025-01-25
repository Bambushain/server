use crate::pages::banner_page::BannerPage;
use bamboo_common::core::entities::get_dependencies;
use chrono::Datelike;
use leptos::prelude::*;
use std::ops::Not;

#[component]
pub fn Licenses() -> impl IntoView {
    let dependencies = get_dependencies();
    let year = chrono::Local::now().year();

    view! {
        <BannerPage title="Lizenzen">
            <p>"Bambushain verwendet diverse Bilder, Schriften und Softwarekomponenten, hier findest du eine Liste mit den Lizenzen"</p>
            <h2>"Bambushains Lizenz"</h2>
            <p>
                "Bambushain ist unter der Open Source MIT Lizenz veröffentlicht, den Code kannst du dir hier anschauen: "
                <a href="https://github.com/Creastina/bambushain">"Github"</a>
            </p>
            <details>
                <summary>"Lizenztext"</summary>
                <pre>
                    {format!("MIT License
    
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
SOFTWARE.")}</pre>
            </details>
            <h2>"Bildlizenzen"</h2>
            <h3>"Hintergrundbild"</h3>
            <p>
                "Bambushain hat ein Hintergrundbild von "<a href="https://unsplash.com">"Unsplash"</a>". Das Bild steht unter der Unsplash Lizenz, die wir unten verlinkt und kopiert haben. Das Bild haben wir auch verlinkt und noch einmal ohne Filter gezeigt."
            </p>
            <h4>"Bild"</h4>
            <figure class="bamboo-preview is--license">
                <img class="bamboo-preview__image is--license" src="/public/assets/background-pandas.webp" />
                <figcaption class="bamboo-preview__caption is--license">
                    <a href="https://unsplash.com/de/fotos/ein-holzsteg-mit-pflanzen-und-baumen-l3lxbhkTApM">"Bildherkunft"</a>
                </figcaption>
            </figure>
            <h4>"Lizenz"</h4>
            <p>
                "Unsplash gewährt Ihnen eine unwiderrufliche, nicht-exklusive, weltweite Urheberrechtslizenz, um Fotos von Unsplash kostenlos herunterzuladen, zu kopieren, zu verändern, zu verbreiten, aufzuführen und zu nutzen, auch für kommerzielle Zwecke, ohne Genehmigung oder Nennung des Fotografen oder Unsplash. Diese Lizenz umfasst nicht das Recht, Fotos von Unsplash zu kompilieren, um einen ähnlichen oder konkurrierenden Service zu replizieren."
            </p>
            <p>
                "Du findest die Unsplash Lizenz hier: "<a href="https://unsplash.com/de/lizenz">"Unsplash Lizenz"</a>
            </p>
            <h3>"Login Hintergrundbild"</h3>
            <p>
                "Bambushain hat ein Login Hintergrundbild von "<a href="https://unsplash.com">"Unsplash"</a>". Das Bild steht unter der Unsplash Lizenz, die wir unten verlinkt und kopiert haben. Das Bild haben wir auch verlinkt und noch einmal ohne Filter gezeigt."
            </p>
            <h4>"Bild"</h4>
            <figure class="bamboo-preview is--license">
                <img class="bamboo-preview__image is--license" src="/public/assets/background.webp" />
                <figcaption class="bamboo-preview__caption is--license">
                    <a href="https://unsplash.com/de/fotos/haus-zwischen-wald-F-6rwMp1M-Q">"Bildherkunft"</a>
                </figcaption>
            </figure>
            <h4>"Lizenz"</h4>
            <p>
                "Unsplash gewährt Ihnen eine unwiderrufliche, nicht-exklusive, weltweite Urheberrechtslizenz, um Fotos von Unsplash kostenlos herunterzuladen, zu kopieren, zu verändern, zu verbreiten, aufzuführen und zu nutzen, auch für kommerzielle Zwecke, ohne Genehmigung oder Nennung des Fotografen oder Unsplash. Diese Lizenz umfasst nicht das Recht, Fotos von Unsplash zu kompilieren, um einen ähnlichen oder konkurrierenden Service zu replizieren."
            </p>
            <p>
                "Du findest die Unsplash Lizenz hier: "<a href="https://unsplash.com/de/lizenz">"Unsplash Lizenz"</a>
            </p>
            <h3>"Logo"</h3>
            <p>
                "Das aktuelle Bambushain Logo ist von "<a href="https://www.svgrepo.com/">"SVG Repo"</a>". Das Bild steht unter der CC0 Lizenz, die wir unten verlinkt und kopiert haben. Das Bild haben wir auch verlinkt und noch einmal ohne Filter gezeigt."
            </p>
            <h4>"Bild"</h4>
            <figure class="bamboo-preview is--license">
                <img class="bamboo-preview__image is--license is--small" src="/public/assets/favicon.svg" />
                <figcaption class="bamboo-preview__caption is--license">
                    <a href="https://www.svgrepo.com/svg/220234/bamboo">
                        "Bildherkunft"
                    </a>
                </figcaption>
            </figure>
            <details>
                <summary>"Lizenztext"</summary>
                <h4>"Public Domain"</h4>
                <p>
                    <em>
                        "(As visual work) This license also might be referred as No copyright or CC0 1.0 Universal PD Dedication on our website."
                    </em>
                </p>
                <p>
                    "The person who associated a work with this deed has dedicated the work to the public domain by waiving all of his or her rights to the work worldwide under copyright law, including all related and neighboring rights, to the extent allowed by law."
                </p>
                <p>
                    "Or, the work consists of simple geometry and is not ineligable for copyright due to Threshold of originality (this threshold might vary depending on different country laws). For an example \"A stick figure, where the head is represented by a circle and other parts represented by straight lines\" is not copyrightable or falls into public domain."
                </p>
                <p>"You are free:"</p>
                <ul>
                    <li>
                        <strong>"to share"</strong>" – to copy, distribute and transmit the work"
                    </li>
                    <li>
                        <strong>"to remix"</strong>" – to adapt the work"
                    </li>
                </ul>
                <p>"Under the following terms:"</p>
                <ul>
                    <li>
                        <strong>"attribution"</strong>" – there is no author or author waived their right, no attribution"
                    </li>
                    <li>
                        <strong>"share alike"</strong>" – If you remix, transform, or build upon the material, you can distribute your work under any license."
                    </li>
                </ul>
                <p>
                    "Unless in the single pages of icons indexed on this website indicates differently, simple icons distributed on this website is subject to public domain or open source."
                </p>
                <p>
                    "To learn more about this license, "<a href="https://creativecommons.org/publicdomain/zero/1.0/deed.en">"check out this page"</a>"."
                </p>
                <p>
                    "Du findest die CC0 Lizenz hier: "<a href="https://www.svgrepo.com/page/licensing/#CC0">"CC0 Lizenz"</a>
                </p>
            </details>
            <h2>"Schriftlizenzen"</h2>
            <h3>"Albert Sans"</h3>
            <p>
                "Die Standardschrift von Bambushain ist "<a href="https://github.com/usted/Albert-Sans">"Albert Sans"</a> ". Albert Sans ist unter der SIL Open Font License lizensiert. Details dazu unten."
            </p>
            <details>
                <summary>"Lizenztext"</summary>
                <pre>
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
                </pre>
            </details>
            <h3>"Urbanist"</h3>
            <p>
                "Für Menüs, Überschriften und ähnliches benutzt Bambushain "<a href="https://github.com/coreyhu/Urbanist">"Urbanist"</a>". Urbanist ist unter der SIL Open Font License lizensiert. Details dazu unten."
            </p>
            <details>
                <summary>"Lizenztext"</summary>
                <pre>
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
                </pre>
            </details>
            <h2>"Softwarelizenzen"</h2>
            {dependencies.into_iter().map(|dependency| view! {
                <h3>{dependency.name}</h3>
                <dl>
                    {dependency.description.is_empty().not().then_some(view! {
                        <dt>"Beschreibung"</dt>
                        <dd>{dependency.description}</dd>
                    })}
                    {dependency.authors.is_empty().not().then_some(view! {
                        <dt>"Autoren"</dt>
                        <dd>{dependency.authors}</dd>
                    })}
                    {dependency.repository.is_empty().not().then_some(view! {
                        <dt>"Quelle"</dt>
                        <dd><a href=dependency.repository>{dependency.repository.clone()}</a></dd>
                    })}
                    {dependency.license.is_empty().not().then_some(view! {
                        <dt>"Lizenz"</dt>
                        <dd>{dependency.license}</dd>
                    })}
                </dl>
            }).collect_view()}
        </BannerPage>
    }
}
