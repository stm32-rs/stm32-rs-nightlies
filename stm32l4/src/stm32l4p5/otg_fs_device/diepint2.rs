#[doc = "Register `DIEPINT2` reader"]
pub type R = crate::R<DIEPINT2rs>;
#[doc = "Register `DIEPINT2` writer"]
pub type W = crate::W<DIEPINT2rs>;
#[doc = "Field `XFRC` reader - XFRC"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - EPDISD"]
pub type EPDISD_R = crate::BitReader;
#[doc = "Field `EPDISD` writer - EPDISD"]
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOC` reader - TOC"]
pub type TOC_R = crate::BitReader;
#[doc = "Field `TOC` writer - TOC"]
pub type TOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFE` reader - ITTXFE"]
pub type ITTXFE_R = crate::BitReader;
#[doc = "Field `ITTXFE` writer - ITTXFE"]
pub type ITTXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNM` reader - IN token received with EP mismatch"]
pub type INEPNM_R = crate::BitReader;
#[doc = "Field `INEPNM` writer - IN token received with EP mismatch"]
pub type INEPNM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNE` reader - INEPNE"]
pub type INEPNE_R = crate::BitReader;
#[doc = "Field `INEPNE` writer - INEPNE"]
pub type INEPNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TXFE_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS` reader - Packet dropped status"]
pub type PKTDRPSTS_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - Packet dropped status"]
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK input"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK input"]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch"]
    #[inline(always)]
    pub fn inepnm(&self) -> INEPNM_R {
        INEPNM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK input"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<DIEPINT2rs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<DIEPINT2rs> {
        EPDISD_W::new(self, 1)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<DIEPINT2rs> {
        TOC_W::new(self, 3)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfe(&mut self) -> ITTXFE_W<DIEPINT2rs> {
        ITTXFE_W::new(self, 4)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn inepnm(&mut self) -> INEPNM_W<DIEPINT2rs> {
        INEPNM_W::new(self, 5)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    #[must_use]
    pub fn inepne(&mut self) -> INEPNE_W<DIEPINT2rs> {
        INEPNE_W::new(self, 6)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<DIEPINT2rs> {
        PKTDRPSTS_W::new(self, 11)
    }
    #[doc = "Bit 13 - NAK input"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<DIEPINT2rs> {
        NAK_W::new(self, 13)
    }
}
#[doc = "device endpoint-2 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINT2rs;
impl crate::RegisterSpec for DIEPINT2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint2::R`](R) reader structure"]
impl crate::Readable for DIEPINT2rs {}
#[doc = "`write(|w| ..)` method takes [`diepint2::W`](W) writer structure"]
impl crate::Writable for DIEPINT2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT2 to value 0x80"]
impl crate::Resettable for DIEPINT2rs {
    const RESET_VALUE: u32 = 0x80;
}
