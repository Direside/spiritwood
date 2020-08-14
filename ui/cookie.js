function getCookie(sName)
{
    sName = sName.toLowerCase();
    var oCrumbles = document.cookie.split(';');
    for(var i=0; i<oCrumbles.length;i++)
    {
        var oPair= oCrumbles[i].split('=');
        var sKey = decodeURIComponent(oPair[0].trim().toLowerCase());
        var sValue = oPair.length>1?oPair[1]:'';
        if(sKey == sName)
            return decodeURIComponent(sValue);
    }
    return '';
}

function setCookie(sName,sValue)
{
    var oDate = new Date();
    oDate.setYear(oDate.getFullYear()+1);
    var sCookie = encodeURIComponent(sName) + '=' + encodeURIComponent(sValue) + ';expires=' + oDate.toGMTString() + ';path=/';
    document.cookie= sCookie;
}

function clearCookie(sName)
{
    setCookie(sName,'');
}

export { getCookie, setCookie, clearCookie }