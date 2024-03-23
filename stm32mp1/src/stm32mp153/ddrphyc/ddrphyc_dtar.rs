#[doc = "Register `DDRPHYC_DTAR` reader"]
pub type R = crate::R<DDRPHYC_DTARrs>;
#[doc = "Register `DDRPHYC_DTAR` writer"]
pub type W = crate::W<DDRPHYC_DTARrs>;
#[doc = "Field `DTCOL` reader - DTCOL"]
pub type DTCOL_R = crate::FieldReader<u16>;
#[doc = "Field `DTCOL` writer - DTCOL"]
pub type DTCOL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DTROW` reader - DTROW"]
pub type DTROW_R = crate::FieldReader<u16>;
#[doc = "Field `DTROW` writer - DTROW"]
pub type DTROW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DTBANK` reader - DTBANK"]
pub type DTBANK_R = crate::FieldReader;
#[doc = "Field `DTBANK` writer - DTBANK"]
pub type DTBANK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DTMPR` reader - DTMPR"]
pub type DTMPR_R = crate::BitReader;
#[doc = "Field `DTMPR` writer - DTMPR"]
pub type DTMPR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - DTCOL"]
    #[inline(always)]
    pub fn dtcol(&self) -> DTCOL_R {
        DTCOL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27 - DTROW"]
    #[inline(always)]
    pub fn dtrow(&self) -> DTROW_R {
        DTROW_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - DTBANK"]
    #[inline(always)]
    pub fn dtbank(&self) -> DTBANK_R {
        DTBANK_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - DTMPR"]
    #[inline(always)]
    pub fn dtmpr(&self) -> DTMPR_R {
        DTMPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - DTCOL"]
    #[inline(always)]
    #[must_use]
    pub fn dtcol(&mut self) -> DTCOL_W<DDRPHYC_DTARrs> {
        DTCOL_W::new(self, 0)
    }
    #[doc = "Bits 12:27 - DTROW"]
    #[inline(always)]
    #[must_use]
    pub fn dtrow(&mut self) -> DTROW_W<DDRPHYC_DTARrs> {
        DTROW_W::new(self, 12)
    }
    #[doc = "Bits 28:30 - DTBANK"]
    #[inline(always)]
    #[must_use]
    pub fn dtbank(&mut self) -> DTBANK_W<DDRPHYC_DTARrs> {
        DTBANK_W::new(self, 28)
    }
    #[doc = "Bit 31 - DTMPR"]
    #[inline(always)]
    #[must_use]
    pub fn dtmpr(&mut self) -> DTMPR_W<DDRPHYC_DTARrs> {
        DTMPR_W::new(self, 31)
    }
}
#[doc = "DDRPHYC DTA register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dtar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dtar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DTARrs;
impl crate::RegisterSpec for DDRPHYC_DTARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dtar::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DTARrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dtar::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DTARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DTAR to value 0"]
impl crate::Resettable for DDRPHYC_DTARrs {
    const RESET_VALUE: u32 = 0;
}
