#[doc = "Register `FDCAN_TTTMK` reader"]
pub type R = crate::R<FDCAN_TTTMKrs>;
#[doc = "Register `FDCAN_TTTMK` writer"]
pub type W = crate::W<FDCAN_TTTMKrs>;
#[doc = "Field `TM` reader - TM"]
pub type TM_R = crate::FieldReader<u16>;
#[doc = "Field `TM` writer - TM"]
pub type TM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TICC` reader - TICC"]
pub type TICC_R = crate::FieldReader;
#[doc = "Field `TICC` writer - TICC"]
pub type TICC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LCKM` reader - LCKM"]
pub type LCKM_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - TM"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - TICC"]
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - LCKM"]
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - TM"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<FDCAN_TTTMKrs> {
        TM_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - TICC"]
    #[inline(always)]
    #[must_use]
    pub fn ticc(&mut self) -> TICC_W<FDCAN_TTTMKrs> {
        TICC_W::new(self, 16)
    }
}
#[doc = "A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tttmk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tttmk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTTMKrs;
impl crate::RegisterSpec for FDCAN_TTTMKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tttmk::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTTMKrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tttmk::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTTMKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTTMK to value 0"]
impl crate::Resettable for FDCAN_TTTMKrs {
    const RESET_VALUE: u32 = 0;
}
