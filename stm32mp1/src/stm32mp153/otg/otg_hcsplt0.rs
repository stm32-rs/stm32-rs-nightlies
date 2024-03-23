#[doc = "Register `OTG_HCSPLT0` reader"]
pub type R = crate::R<OTG_HCSPLT0rs>;
#[doc = "Register `OTG_HCSPLT0` writer"]
pub type W = crate::W<OTG_HCSPLT0rs>;
#[doc = "Field `PRTADDR` reader - PRTADDR"]
pub type PRTADDR_R = crate::FieldReader;
#[doc = "Field `PRTADDR` writer - PRTADDR"]
pub type PRTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HUBADDR` reader - HUBADDR"]
pub type HUBADDR_R = crate::FieldReader;
#[doc = "Field `HUBADDR` writer - HUBADDR"]
pub type HUBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XACTPOS` reader - XACTPOS"]
pub type XACTPOS_R = crate::FieldReader;
#[doc = "Field `XACTPOS` writer - XACTPOS"]
pub type XACTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPLSPLT` reader - COMPLSPLT"]
pub type COMPLSPLT_R = crate::BitReader;
#[doc = "Field `COMPLSPLT` writer - COMPLSPLT"]
pub type COMPLSPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLITEN` reader - SPLITEN"]
pub type SPLITEN_R = crate::BitReader;
#[doc = "Field `SPLITEN` writer - SPLITEN"]
pub type SPLITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - PRTADDR"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - HUBADDR"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - COMPLSPLT"]
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - SPLITEN"]
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PRTADDR"]
    #[inline(always)]
    #[must_use]
    pub fn prtaddr(&mut self) -> PRTADDR_W<OTG_HCSPLT0rs> {
        PRTADDR_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - HUBADDR"]
    #[inline(always)]
    #[must_use]
    pub fn hubaddr(&mut self) -> HUBADDR_W<OTG_HCSPLT0rs> {
        HUBADDR_W::new(self, 7)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    #[must_use]
    pub fn xactpos(&mut self) -> XACTPOS_W<OTG_HCSPLT0rs> {
        XACTPOS_W::new(self, 14)
    }
    #[doc = "Bit 16 - COMPLSPLT"]
    #[inline(always)]
    #[must_use]
    pub fn complsplt(&mut self) -> COMPLSPLT_W<OTG_HCSPLT0rs> {
        COMPLSPLT_W::new(self, 16)
    }
    #[doc = "Bit 31 - SPLITEN"]
    #[inline(always)]
    #[must_use]
    pub fn spliten(&mut self) -> SPLITEN_W<OTG_HCSPLT0rs> {
        SPLITEN_W::new(self, 31)
    }
}
#[doc = "OTG host channel 0 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HCSPLT0rs;
impl crate::RegisterSpec for OTG_HCSPLT0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hcsplt0::R`](R) reader structure"]
impl crate::Readable for OTG_HCSPLT0rs {}
#[doc = "`write(|w| ..)` method takes [`otg_hcsplt0::W`](W) writer structure"]
impl crate::Writable for OTG_HCSPLT0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HCSPLT0 to value 0"]
impl crate::Resettable for OTG_HCSPLT0rs {
    const RESET_VALUE: u32 = 0;
}
