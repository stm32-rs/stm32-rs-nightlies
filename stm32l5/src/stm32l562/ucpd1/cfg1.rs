#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1rs>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1rs>;
#[doc = "Field `HBITCLKDIV` reader - HBITCLKDIV"]
pub type HBITCLKDIV_R = crate::FieldReader;
#[doc = "Field `HBITCLKDIV` writer - HBITCLKDIV"]
pub type HBITCLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IFRGAP` reader - IFRGAP"]
pub type IFRGAP_R = crate::FieldReader;
#[doc = "Field `IFRGAP` writer - IFRGAP"]
pub type IFRGAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRANSWIN` reader - TRANSWIN"]
pub type TRANSWIN_R = crate::FieldReader;
#[doc = "Field `TRANSWIN` writer - TRANSWIN"]
pub type TRANSWIN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PSC_USBPDCLK` reader - PSC_USBPDCLK"]
pub type PSC_USBPDCLK_R = crate::FieldReader;
#[doc = "Field `PSC_USBPDCLK` writer - PSC_USBPDCLK"]
pub type PSC_USBPDCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXORDSETEN` reader - RXORDSETEN"]
pub type RXORDSETEN_R = crate::FieldReader<u16>;
#[doc = "Field `RXORDSETEN` writer - RXORDSETEN"]
pub type RXORDSETEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TXDMAEN` reader - TXDMAEN"]
pub type TXDMAEN_R = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - TXDMAEN"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAEN` reader - RXDMAEN:"]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - RXDMAEN:"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPDEN` reader - UCPDEN"]
pub type UCPDEN_R = crate::BitReader;
#[doc = "Field `UCPDEN` writer - UCPDEN"]
pub type UCPDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - HBITCLKDIV"]
    #[inline(always)]
    pub fn hbitclkdiv(&self) -> HBITCLKDIV_R {
        HBITCLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - IFRGAP"]
    #[inline(always)]
    pub fn ifrgap(&self) -> IFRGAP_R {
        IFRGAP_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - TRANSWIN"]
    #[inline(always)]
    pub fn transwin(&self) -> TRANSWIN_R {
        TRANSWIN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19 - PSC_USBPDCLK"]
    #[inline(always)]
    pub fn psc_usbpdclk(&self) -> PSC_USBPDCLK_R {
        PSC_USBPDCLK_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:28 - RXORDSETEN"]
    #[inline(always)]
    pub fn rxordseten(&self) -> RXORDSETEN_R {
        RXORDSETEN_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RXDMAEN:"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UCPDEN"]
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - HBITCLKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn hbitclkdiv(&mut self) -> HBITCLKDIV_W<CFG1rs> {
        HBITCLKDIV_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - IFRGAP"]
    #[inline(always)]
    #[must_use]
    pub fn ifrgap(&mut self) -> IFRGAP_W<CFG1rs> {
        IFRGAP_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - TRANSWIN"]
    #[inline(always)]
    #[must_use]
    pub fn transwin(&mut self) -> TRANSWIN_W<CFG1rs> {
        TRANSWIN_W::new(self, 11)
    }
    #[doc = "Bits 17:19 - PSC_USBPDCLK"]
    #[inline(always)]
    #[must_use]
    pub fn psc_usbpdclk(&mut self) -> PSC_USBPDCLK_W<CFG1rs> {
        PSC_USBPDCLK_W::new(self, 17)
    }
    #[doc = "Bits 20:28 - RXORDSETEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxordseten(&mut self) -> RXORDSETEN_W<CFG1rs> {
        RXORDSETEN_W::new(self, 20)
    }
    #[doc = "Bit 29 - TXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CFG1rs> {
        TXDMAEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - RXDMAEN:"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CFG1rs> {
        RXDMAEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - UCPDEN"]
    #[inline(always)]
    #[must_use]
    pub fn ucpden(&mut self) -> UCPDEN_W<CFG1rs> {
        UCPDEN_W::new(self, 31)
    }
}
#[doc = "UCPD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1rs;
impl crate::RegisterSpec for CFG1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1rs {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1rs {
    const RESET_VALUE: u32 = 0;
}
