#[doc = "Register `GADPCTL` reader"]
pub type R = crate::R<GADPCTLrs>;
#[doc = "Register `GADPCTL` writer"]
pub type W = crate::W<GADPCTLrs>;
#[doc = "Field `PRBDSCHG` reader - Probe discharge"]
pub type PRBDSCHG_R = crate::FieldReader;
#[doc = "Field `PRBDSCHG` writer - Probe discharge"]
pub type PRBDSCHG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRBDELTA` reader - Probe delta"]
pub type PRBDELTA_R = crate::FieldReader;
#[doc = "Field `PRBDELTA` writer - Probe delta"]
pub type PRBDELTA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRBPER` reader - Probe period"]
pub type PRBPER_R = crate::FieldReader;
#[doc = "Field `PRBPER` writer - Probe period"]
pub type PRBPER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTIM` reader - Ramp time"]
pub type RTIM_R = crate::FieldReader<u16>;
#[doc = "Field `ENAPRB` reader - Enable probe"]
pub type ENAPRB_R = crate::BitReader;
#[doc = "Field `ENAPRB` writer - Enable probe"]
pub type ENAPRB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENASNS` reader - Enable sense"]
pub type ENASNS_R = crate::BitReader;
#[doc = "Field `ENASNS` writer - Enable sense"]
pub type ENASNS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADPRST` reader - ADP reset"]
pub type ADPRST_R = crate::BitReader;
#[doc = "Field `ADPEN` reader - ADP enable"]
pub type ADPEN_R = crate::BitReader;
#[doc = "Field `ADPEN` writer - ADP enable"]
pub type ADPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADPPRBIF` reader - ADP probe interrupt flag"]
pub type ADPPRBIF_R = crate::BitReader;
#[doc = "Field `ADPPRBIF` writer - ADP probe interrupt flag"]
pub type ADPPRBIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADPSNSIF` reader - ADP sense interrupt flag"]
pub type ADPSNSIF_R = crate::BitReader;
#[doc = "Field `ADPSNSIF` writer - ADP sense interrupt flag"]
pub type ADPSNSIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADPTOIF` reader - ADP timeout interrupt flag"]
pub type ADPTOIF_R = crate::BitReader;
#[doc = "Field `ADPTOIF` writer - ADP timeout interrupt flag"]
pub type ADPTOIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADPPRBIM` reader - ADP probe interrupt mask"]
pub type ADPPRBIM_R = crate::BitReader;
#[doc = "Field `ADPPRBIM` writer - ADP probe interrupt mask"]
pub type ADPPRBIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADPSNSIM` reader - ADP sense interrupt mask"]
pub type ADPSNSIM_R = crate::BitReader;
#[doc = "Field `ADPSNSIM` writer - ADP sense interrupt mask"]
pub type ADPSNSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADPTOIM` reader - ADP timeout interrupt mask"]
pub type ADPTOIM_R = crate::BitReader;
#[doc = "Field `ADPTOIM` writer - ADP timeout interrupt mask"]
pub type ADPTOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR` reader - Access request"]
pub type AR_R = crate::FieldReader;
#[doc = "Field `AR` writer - Access request"]
pub type AR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Probe discharge"]
    #[inline(always)]
    pub fn prbdschg(&self) -> PRBDSCHG_R {
        PRBDSCHG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Probe delta"]
    #[inline(always)]
    pub fn prbdelta(&self) -> PRBDELTA_R {
        PRBDELTA_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Probe period"]
    #[inline(always)]
    pub fn prbper(&self) -> PRBPER_R {
        PRBPER_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:16 - Ramp time"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x07ff) as u16)
    }
    #[doc = "Bit 17 - Enable probe"]
    #[inline(always)]
    pub fn enaprb(&self) -> ENAPRB_R {
        ENAPRB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable sense"]
    #[inline(always)]
    pub fn enasns(&self) -> ENASNS_R {
        ENASNS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADP reset"]
    #[inline(always)]
    pub fn adprst(&self) -> ADPRST_R {
        ADPRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADP enable"]
    #[inline(always)]
    pub fn adpen(&self) -> ADPEN_R {
        ADPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADP probe interrupt flag"]
    #[inline(always)]
    pub fn adpprbif(&self) -> ADPPRBIF_R {
        ADPPRBIF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADP sense interrupt flag"]
    #[inline(always)]
    pub fn adpsnsif(&self) -> ADPSNSIF_R {
        ADPSNSIF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADP timeout interrupt flag"]
    #[inline(always)]
    pub fn adptoif(&self) -> ADPTOIF_R {
        ADPTOIF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADP probe interrupt mask"]
    #[inline(always)]
    pub fn adpprbim(&self) -> ADPPRBIM_R {
        ADPPRBIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADP sense interrupt mask"]
    #[inline(always)]
    pub fn adpsnsim(&self) -> ADPSNSIM_R {
        ADPSNSIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADP timeout interrupt mask"]
    #[inline(always)]
    pub fn adptoim(&self) -> ADPTOIM_R {
        ADPTOIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Access request"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Probe discharge"]
    #[inline(always)]
    #[must_use]
    pub fn prbdschg(&mut self) -> PRBDSCHG_W<GADPCTLrs> {
        PRBDSCHG_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Probe delta"]
    #[inline(always)]
    #[must_use]
    pub fn prbdelta(&mut self) -> PRBDELTA_W<GADPCTLrs> {
        PRBDELTA_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Probe period"]
    #[inline(always)]
    #[must_use]
    pub fn prbper(&mut self) -> PRBPER_W<GADPCTLrs> {
        PRBPER_W::new(self, 4)
    }
    #[doc = "Bit 17 - Enable probe"]
    #[inline(always)]
    #[must_use]
    pub fn enaprb(&mut self) -> ENAPRB_W<GADPCTLrs> {
        ENAPRB_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable sense"]
    #[inline(always)]
    #[must_use]
    pub fn enasns(&mut self) -> ENASNS_W<GADPCTLrs> {
        ENASNS_W::new(self, 18)
    }
    #[doc = "Bit 20 - ADP enable"]
    #[inline(always)]
    #[must_use]
    pub fn adpen(&mut self) -> ADPEN_W<GADPCTLrs> {
        ADPEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - ADP probe interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpprbif(&mut self) -> ADPPRBIF_W<GADPCTLrs> {
        ADPPRBIF_W::new(self, 21)
    }
    #[doc = "Bit 22 - ADP sense interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adpsnsif(&mut self) -> ADPSNSIF_W<GADPCTLrs> {
        ADPSNSIF_W::new(self, 22)
    }
    #[doc = "Bit 23 - ADP timeout interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn adptoif(&mut self) -> ADPTOIF_W<GADPCTLrs> {
        ADPTOIF_W::new(self, 23)
    }
    #[doc = "Bit 24 - ADP probe interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn adpprbim(&mut self) -> ADPPRBIM_W<GADPCTLrs> {
        ADPPRBIM_W::new(self, 24)
    }
    #[doc = "Bit 25 - ADP sense interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn adpsnsim(&mut self) -> ADPSNSIM_W<GADPCTLrs> {
        ADPSNSIM_W::new(self, 25)
    }
    #[doc = "Bit 26 - ADP timeout interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn adptoim(&mut self) -> ADPTOIM_W<GADPCTLrs> {
        ADPTOIM_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - Access request"]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<GADPCTLrs> {
        AR_W::new(self, 27)
    }
}
#[doc = "OTG ADP timer, control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gadpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gadpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GADPCTLrs;
impl crate::RegisterSpec for GADPCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gadpctl::R`](R) reader structure"]
impl crate::Readable for GADPCTLrs {}
#[doc = "`write(|w| ..)` method takes [`gadpctl::W`](W) writer structure"]
impl crate::Writable for GADPCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GADPCTL to value 0x0200_0400"]
impl crate::Resettable for GADPCTLrs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
