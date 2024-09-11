pub struct Integration {
    pub name: &'static str,
    pub image: &'static str,
}

pub const INTEGRATIONS: [Integration; 16] = [Integration {
  name: "ring",
  image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTIr91eAi3NC85wLntkOtCVTHPrrmK3gbvHcLASAbbJiOlqX4dTxttliz8uDi8mDfcRTzI&usqp=CAU"
},Integration{
  name: "eufy",
  image: "data:image/png;base64,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",
}, Integration{
  name: "alexa",
  image: "https://upload.wikimedia.org/wikipedia/commons/c/cc/Amazon_Alexa_App_Logo.png",
}, Integration{
  name: "tuya",
  image: "data:image/png;base64,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",
}, Integration{
  name: "openai",
  image:"https://play-lh.googleusercontent.com/6qi3w4uqKaD1c-CBdkkfO6IL0lH4OoCTEdiX0oYbLFxwfvxu1t8vuwHcagdYSFmFKmI=w480-h960-rw",
}, Integration{
  name: "govee",
  image: "data:image/png;base64,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",
}, Integration {
  name: "instacart",
  image: "https://imgs.search.brave.com/jT5Tgh8vw5Z9G_v6gw6Y5yyOxyBu0QEasi9tjeLx7A8/rs:fit:500:0:0/g:ce/aHR0cHM6Ly9yZWVv/by5qaW5nb2ZmZXIu/Y29tL0luc3RhY2Fy/dC1pY29uLnBuZyFp/Y29uNTEy",
}, Integration {
  name: "stoplight",
  image: "https://imgs.search.brave.com/r7A61_F6ELOOGWb6_0nsadT-2V3w4XxCLN4bBaq9UzQ/rs:fit:860:0:0/g:ce/aHR0cHM6Ly93d3cu/aWNvbnNob2NrLmNv/bS9pbWFnZS9CZXRh/L0dlbmVyYWwvdHJh/ZmZpY19saWdodA",
}, Integration {
  name: "roku",
  image: "data:image/png;base64,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",
},Integration {
  name: "simpli safe",
  image: "https://play-lh.googleusercontent.com/QSJQlcF2wMi1QDa_6skQd6grZglVwGuKel4fTjJ054LAJzY5Z2HqHOjpjluhm1VkDGli=w480-h960-rw",
},Integration {
  name: "tp-link kasa",
  image: "https://play-lh.googleusercontent.com/HH2EMJy6xdJX9WM72G5LJ8SRzACsxCSjPKCNYiHdNuSiij1M4v5W-3XLzXVXVuhWnKA=w480-h960-rw",
}, Integration {
  name: "cync",
  image: "https://play-lh.googleusercontent.com/Hso3u15eqsC4wgP5ccaaNf0RpolVtXTeZr_pNyjoyWcwyR91BUI5cTeratOuUtrq7w=w480-h960-rw"
}, Integration {
  name: "chromecast0",
  image: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAAAXNSR0IArs4c6QAABNdJREFUaAXtWV1om1UYPu+XxFZIoqMyprubMBUvRBH219YkeKOC4mB6oSAIzhuHf3QKTbtvayK6Dn+vFC/HLlZ/hjCQdSTZqp3TOtELYYJeCFbRjnZL5laT77w+p+05nO8zW9L6pe0ggXLe857Tc57nPe9fWiHan7YF2hZoW2AlLUD25VtyU+s7qWO/YEoTiZvttVUgT7Lg0mWe3X0qe9NvGo8hoMBfTx3fC0FdenF1jnzuEs/epUk4GmSn6Bhe/eAVWuqa85IF4IYAFlILutU/wMU1yKgWgj5f6I8b99J7VnLM5Cus77exWi+gl6+tsU1gpd/LxEAjILYPNtobxnqzMXjNu5B5gWYZh2HdMM9o/gVY9DHz2TAvD+OspgkUsvEDxWzids+Tvczi8zAuD+OMpgnoy04MJseK2fgDXo0z0H2j9Ss1mhhI58plVLhJWHdSzI+/CCGPlrI3nAY4UwU10BN7EkXhupvTsZdeEcLZi7Id02vLOZp24UppEoT+AP7PmGmkNBA/Xg9c777puyNOdISIbq23vhRdMKkE8en1hi6EV1kHYDsdR4xm8uWTqdz5zUFAJwfXfDdbE1uhX3aXakjAD5Z6HIqcSufLH3e7MxvstXE38efUdAVdIo/a+lbLhsDs+UvJf6ryNik5zcJ7Cv5/EN+ApusBIEHbY7HIRGqocr+9/sOBdRenpi8+Ct2yvYSJARuIllNuMepE7u1FUD8niBSw4McTkl8sDCTesxe2uuW1HVEx/n9iQvu4PndJMVBy0zWAKxSyie2C5Ta8yJf6wIUxIhx6F7Hxtq1X7uTJ2g6krqqtb4VsXKjR4YVscrzYn+iWUryAvZ5/Pz2fGSrvsnUqsOGGe2xdK2RDQNUBWPIsfgqQ9/bkZ+6pdyFS6TtC1h5EWzHjW3forWBMFKtvvoE9LY0HQwD+Gsf34o34SUMejInot3OEcuUdPqCYFAZuPFariW0BEhFy+LAvO7muRMVGoWvdxxCofwUIER1O5ytf+YBh85ib+JHYexyicSdkpzXITsP2Wapit7J3akBgHgpS1abrYpGvU0PllA1OvQRi4mVbp1LsffsubLF1UsrX7HmYsiEgq38nVB1gTzwhmEf+m0Goixw6FiShYiKYnRyH9tsgVQPYqlbcECi5aytfuMmfioPxQ0ibj5Gs3QEin9pA8BIxtBQfBd2JmHf79hF1BwMaL/OhvScs+aqFTF2Szl/oI3ZeRzEzZPE6p4v9cV9PlMmVP7GLHSz+Ab4/PBsW0CUVMnV5sT85zCRftYGomABgX3byPOGrxshmD+N3GhrIPncpsrGqqgMq26BR2xm8WJEIuhOg5ewLiSfG7N5JdbHoXDfZe1ohGwKqDijLwlffB5njPfm//H9eZ6/PH9i00S52qu0gwUf9IJ2H/PPwZ+YbmX00yGSi3HkQOtVtAvdc8foZbnMEfm5cJ8qRR7B0Rq2rD15gFEZ4cn4G/yGxQct6DPqy1i9ynNT7zQtohR4VCbjTM3quRpZ0xJ4DYI89Z+n8as/B6BbfPKQJDFXSR12RwPwGelpvVGNVygl7Dnm9Pa95bCyj9CDYAgJ8Tv2XRt97dQLMd+qNaoyKyz6AaBF8ABut22ctVsZdv8Pyh+z/ziz2jPb+tgXaFmhbIHwL/Auw88OIDWdgOwAAAABJRU5ErkJggg==",
}, Integration {
  name: "chromecast1",
  image: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAAAXNSR0IArs4c6QAABNdJREFUaAXtWV1om1UYPu+XxFZIoqMyprubMBUvRBH219YkeKOC4mB6oSAIzhuHf3QKTbtvayK6Dn+vFC/HLlZ/hjCQdSTZqp3TOtELYYJeCFbRjnZL5laT77w+p+05nO8zW9L6pe0ggXLe857Tc57nPe9fWiHan7YF2hZoW2AlLUD25VtyU+s7qWO/YEoTiZvttVUgT7Lg0mWe3X0qe9NvGo8hoMBfTx3fC0FdenF1jnzuEs/epUk4GmSn6Bhe/eAVWuqa85IF4IYAFlILutU/wMU1yKgWgj5f6I8b99J7VnLM5Cus77exWi+gl6+tsU1gpd/LxEAjILYPNtobxnqzMXjNu5B5gWYZh2HdMM9o/gVY9DHz2TAvD+OspgkUsvEDxWzids+Tvczi8zAuD+OMpgnoy04MJseK2fgDXo0z0H2j9Ss1mhhI58plVLhJWHdSzI+/CCGPlrI3nAY4UwU10BN7EkXhupvTsZdeEcLZi7Id02vLOZp24UppEoT+AP7PmGmkNBA/Xg9c777puyNOdISIbq23vhRdMKkE8en1hi6EV1kHYDsdR4xm8uWTqdz5zUFAJwfXfDdbE1uhX3aXakjAD5Z6HIqcSufLH3e7MxvstXE38efUdAVdIo/a+lbLhsDs+UvJf6ryNik5zcJ7Cv5/EN+ApusBIEHbY7HIRGqocr+9/sOBdRenpi8+Ct2yvYSJARuIllNuMepE7u1FUD8niBSw4McTkl8sDCTesxe2uuW1HVEx/n9iQvu4PndJMVBy0zWAKxSyie2C5Ta8yJf6wIUxIhx6F7Hxtq1X7uTJ2g6krqqtb4VsXKjR4YVscrzYn+iWUryAvZ5/Pz2fGSrvsnUqsOGGe2xdK2RDQNUBWPIsfgqQ9/bkZ+6pdyFS6TtC1h5EWzHjW3forWBMFKtvvoE9LY0HQwD+Gsf34o34SUMejInot3OEcuUdPqCYFAZuPFariW0BEhFy+LAvO7muRMVGoWvdxxCofwUIER1O5ytf+YBh85ib+JHYexyicSdkpzXITsP2Wapit7J3akBgHgpS1abrYpGvU0PllA1OvQRi4mVbp1LsffsubLF1UsrX7HmYsiEgq38nVB1gTzwhmEf+m0Goixw6FiShYiKYnRyH9tsgVQPYqlbcECi5aytfuMmfioPxQ0ibj5Gs3QEin9pA8BIxtBQfBd2JmHf79hF1BwMaL/OhvScs+aqFTF2Szl/oI3ZeRzEzZPE6p4v9cV9PlMmVP7GLHSz+Ab4/PBsW0CUVMnV5sT85zCRftYGomABgX3byPOGrxshmD+N3GhrIPncpsrGqqgMq26BR2xm8WJEIuhOg5ewLiSfG7N5JdbHoXDfZe1ohGwKqDijLwlffB5njPfm//H9eZ6/PH9i00S52qu0gwUf9IJ2H/PPwZ+YbmX00yGSi3HkQOtVtAvdc8foZbnMEfm5cJ8qRR7B0Rq2rD15gFEZ4cn4G/yGxQct6DPqy1i9ynNT7zQtohR4VCbjTM3quRpZ0xJ4DYI89Z+n8as/B6BbfPKQJDFXSR12RwPwGelpvVGNVygl7Dnm9Pa95bCyj9CDYAgJ8Tv2XRt97dQLMd+qNaoyKyz6AaBF8ABut22ctVsZdv8Pyh+z/ziz2jPb+tgXaFmhbIHwL/Auw88OIDWdgOwAAAABJRU5ErkJggg==",
}, Integration {
  name: "chromecast2",
  image: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAAAXNSR0IArs4c6QAABNdJREFUaAXtWV1om1UYPu+XxFZIoqMyprubMBUvRBH219YkeKOC4mB6oSAIzhuHf3QKTbtvayK6Dn+vFC/HLlZ/hjCQdSTZqp3TOtELYYJeCFbRjnZL5laT77w+p+05nO8zW9L6pe0ggXLe857Tc57nPe9fWiHan7YF2hZoW2AlLUD25VtyU+s7qWO/YEoTiZvttVUgT7Lg0mWe3X0qe9NvGo8hoMBfTx3fC0FdenF1jnzuEs/epUk4GmSn6Bhe/eAVWuqa85IF4IYAFlILutU/wMU1yKgWgj5f6I8b99J7VnLM5Cus77exWi+gl6+tsU1gpd/LxEAjILYPNtobxnqzMXjNu5B5gWYZh2HdMM9o/gVY9DHz2TAvD+OspgkUsvEDxWzids+Tvczi8zAuD+OMpgnoy04MJseK2fgDXo0z0H2j9Ss1mhhI58plVLhJWHdSzI+/CCGPlrI3nAY4UwU10BN7EkXhupvTsZdeEcLZi7Id02vLOZp24UppEoT+AP7PmGmkNBA/Xg9c777puyNOdISIbq23vhRdMKkE8en1hi6EV1kHYDsdR4xm8uWTqdz5zUFAJwfXfDdbE1uhX3aXakjAD5Z6HIqcSufLH3e7MxvstXE38efUdAVdIo/a+lbLhsDs+UvJf6ryNik5zcJ7Cv5/EN+ApusBIEHbY7HIRGqocr+9/sOBdRenpi8+Ct2yvYSJARuIllNuMepE7u1FUD8niBSw4McTkl8sDCTesxe2uuW1HVEx/n9iQvu4PndJMVBy0zWAKxSyie2C5Ta8yJf6wIUxIhx6F7Hxtq1X7uTJ2g6krqqtb4VsXKjR4YVscrzYn+iWUryAvZ5/Pz2fGSrvsnUqsOGGe2xdK2RDQNUBWPIsfgqQ9/bkZ+6pdyFS6TtC1h5EWzHjW3forWBMFKtvvoE9LY0HQwD+Gsf34o34SUMejInot3OEcuUdPqCYFAZuPFariW0BEhFy+LAvO7muRMVGoWvdxxCofwUIER1O5ytf+YBh85ib+JHYexyicSdkpzXITsP2Wapit7J3akBgHgpS1abrYpGvU0PllA1OvQRi4mVbp1LsffsubLF1UsrX7HmYsiEgq38nVB1gTzwhmEf+m0Goixw6FiShYiKYnRyH9tsgVQPYqlbcECi5aytfuMmfioPxQ0ibj5Gs3QEin9pA8BIxtBQfBd2JmHf79hF1BwMaL/OhvScs+aqFTF2Szl/oI3ZeRzEzZPE6p4v9cV9PlMmVP7GLHSz+Ab4/PBsW0CUVMnV5sT85zCRftYGomABgX3byPOGrxshmD+N3GhrIPncpsrGqqgMq26BR2xm8WJEIuhOg5ewLiSfG7N5JdbHoXDfZe1ohGwKqDijLwlffB5njPfm//H9eZ6/PH9i00S52qu0gwUf9IJ2H/PPwZ+YbmX00yGSi3HkQOtVtAvdc8foZbnMEfm5cJ8qRR7B0Rq2rD15gFEZ4cn4G/yGxQct6DPqy1i9ynNT7zQtohR4VCbjTM3quRpZ0xJ4DYI89Z+n8as/B6BbfPKQJDFXSR12RwPwGelpvVGNVygl7Dnm9Pa95bCyj9CDYAgJ8Tv2XRt97dQLMd+qNaoyKyz6AaBF8ABut22ctVsZdv8Pyh+z/ziz2jPb+tgXaFmhbIHwL/Auw88OIDWdgOwAAAABJRU5ErkJggg==",
}, Integration {
  name: "chromecast3",
  image: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAAAXNSR0IArs4c6QAABNdJREFUaAXtWV1om1UYPu+XxFZIoqMyprubMBUvRBH219YkeKOC4mB6oSAIzhuHf3QKTbtvayK6Dn+vFC/HLlZ/hjCQdSTZqp3TOtELYYJeCFbRjnZL5laT77w+p+05nO8zW9L6pe0ggXLe857Tc57nPe9fWiHan7YF2hZoW2AlLUD25VtyU+s7qWO/YEoTiZvttVUgT7Lg0mWe3X0qe9NvGo8hoMBfTx3fC0FdenF1jnzuEs/epUk4GmSn6Bhe/eAVWuqa85IF4IYAFlILutU/wMU1yKgWgj5f6I8b99J7VnLM5Cus77exWi+gl6+tsU1gpd/LxEAjILYPNtobxnqzMXjNu5B5gWYZh2HdMM9o/gVY9DHz2TAvD+OspgkUsvEDxWzids+Tvczi8zAuD+OMpgnoy04MJseK2fgDXo0z0H2j9Ss1mhhI58plVLhJWHdSzI+/CCGPlrI3nAY4UwU10BN7EkXhupvTsZdeEcLZi7Id02vLOZp24UppEoT+AP7PmGmkNBA/Xg9c777puyNOdISIbq23vhRdMKkE8en1hi6EV1kHYDsdR4xm8uWTqdz5zUFAJwfXfDdbE1uhX3aXakjAD5Z6HIqcSufLH3e7MxvstXE38efUdAVdIo/a+lbLhsDs+UvJf6ryNik5zcJ7Cv5/EN+ApusBIEHbY7HIRGqocr+9/sOBdRenpi8+Ct2yvYSJARuIllNuMepE7u1FUD8niBSw4McTkl8sDCTesxe2uuW1HVEx/n9iQvu4PndJMVBy0zWAKxSyie2C5Ta8yJf6wIUxIhx6F7Hxtq1X7uTJ2g6krqqtb4VsXKjR4YVscrzYn+iWUryAvZ5/Pz2fGSrvsnUqsOGGe2xdK2RDQNUBWPIsfgqQ9/bkZ+6pdyFS6TtC1h5EWzHjW3forWBMFKtvvoE9LY0HQwD+Gsf34o34SUMejInot3OEcuUdPqCYFAZuPFariW0BEhFy+LAvO7muRMVGoWvdxxCofwUIER1O5ytf+YBh85ib+JHYexyicSdkpzXITsP2Wapit7J3akBgHgpS1abrYpGvU0PllA1OvQRi4mVbp1LsffsubLF1UsrX7HmYsiEgq38nVB1gTzwhmEf+m0Goixw6FiShYiKYnRyH9tsgVQPYqlbcECi5aytfuMmfioPxQ0ibj5Gs3QEin9pA8BIxtBQfBd2JmHf79hF1BwMaL/OhvScs+aqFTF2Szl/oI3ZeRzEzZPE6p4v9cV9PlMmVP7GLHSz+Ab4/PBsW0CUVMnV5sT85zCRftYGomABgX3byPOGrxshmD+N3GhrIPncpsrGqqgMq26BR2xm8WJEIuhOg5ewLiSfG7N5JdbHoXDfZe1ohGwKqDijLwlffB5njPfm//H9eZ6/PH9i00S52qu0gwUf9IJ2H/PPwZ+YbmX00yGSi3HkQOtVtAvdc8foZbnMEfm5cJ8qRR7B0Rq2rD15gFEZ4cn4G/yGxQct6DPqy1i9ynNT7zQtohR4VCbjTM3quRpZ0xJ4DYI89Z+n8as/B6BbfPKQJDFXSR12RwPwGelpvVGNVygl7Dnm9Pa95bCyj9CDYAgJ8Tv2XRt97dQLMd+qNaoyKyz6AaBF8ABut22ctVsZdv8Pyh+z/ziz2jPb+tgXaFmhbIHwL/Auw88OIDWdgOwAAAABJRU5ErkJggg==",
}
];
