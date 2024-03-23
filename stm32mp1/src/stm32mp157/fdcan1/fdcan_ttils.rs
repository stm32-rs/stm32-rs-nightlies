#[doc = "Register `FDCAN_TTILS` reader"]
pub type R = crate::R<FDCAN_TTILSrs>;
#[doc = "Register `FDCAN_TTILS` writer"]
pub type W = crate::W<FDCAN_TTILSrs>;
#[doc = "Field `SBCL` reader - SBCL"]
pub type SBCL_R = crate::BitReader;
#[doc = "Field `SBCL` writer - SBCL"]
pub type SBCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMCL` reader - SMCL"]
pub type SMCL_R = crate::BitReader;
#[doc = "Field `SMCL` writer - SMCL"]
pub type SMCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSML` reader - CSML"]
pub type CSML_R = crate::BitReader;
#[doc = "Field `CSML` writer - CSML"]
pub type CSML_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOGL` reader - SOGL"]
pub type SOGL_R = crate::BitReader;
#[doc = "Field `SOGL` writer - SOGL"]
pub type SOGL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTMIL` reader - RTMIL"]
pub type RTMIL_R = crate::BitReader;
#[doc = "Field `RTMIL` writer - RTMIL"]
pub type RTMIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTMIL` reader - TTMIL"]
pub type TTMIL_R = crate::BitReader;
#[doc = "Field `TTMIL` writer - TTMIL"]
pub type TTMIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEL` reader - SWEL"]
pub type SWEL_R = crate::BitReader;
#[doc = "Field `SWEL` writer - SWEL"]
pub type SWEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTWL` reader - GTWL"]
pub type GTWL_R = crate::BitReader;
#[doc = "Field `GTWL` writer - GTWL"]
pub type GTWL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTDL` reader - GTDL"]
pub type GTDL_R = crate::BitReader;
#[doc = "Field `GTDL` writer - GTDL"]
pub type GTDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTEL` reader - GTEL"]
pub type GTEL_R = crate::BitReader;
#[doc = "Field `GTEL` writer - GTEL"]
pub type GTEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUL` reader - TXUL"]
pub type TXUL_R = crate::BitReader;
#[doc = "Field `TXUL` writer - TXUL"]
pub type TXUL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOL` reader - TXOL"]
pub type TXOL_R = crate::BitReader;
#[doc = "Field `TXOL` writer - TXOL"]
pub type TXOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE1L` reader - SE1L"]
pub type SE1L_R = crate::BitReader;
#[doc = "Field `SE1L` writer - SE1L"]
pub type SE1L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE2L` reader - SE2L"]
pub type SE2L_R = crate::BitReader;
#[doc = "Field `SE2L` writer - SE2L"]
pub type SE2L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELCL` reader - ELCL"]
pub type ELCL_R = crate::BitReader;
#[doc = "Field `ELCL` writer - ELCL"]
pub type ELCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWTL` reader - IWTL"]
pub type IWTL_R = crate::BitReader;
#[doc = "Field `IWTL` writer - IWTL"]
pub type IWTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTL` reader - WTL"]
pub type WTL_R = crate::BitReader;
#[doc = "Field `WTL` writer - WTL"]
pub type WTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWL` reader - AWL"]
pub type AWL_R = crate::BitReader;
#[doc = "Field `AWL` writer - AWL"]
pub type AWL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERL` reader - CERL"]
pub type CERL_R = crate::BitReader;
#[doc = "Field `CERL` writer - CERL"]
pub type CERL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SBCL"]
    #[inline(always)]
    pub fn sbcl(&self) -> SBCL_R {
        SBCL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMCL"]
    #[inline(always)]
    pub fn smcl(&self) -> SMCL_R {
        SMCL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSML"]
    #[inline(always)]
    pub fn csml(&self) -> CSML_R {
        CSML_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOGL"]
    #[inline(always)]
    pub fn sogl(&self) -> SOGL_R {
        SOGL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTMIL"]
    #[inline(always)]
    pub fn rtmil(&self) -> RTMIL_R {
        RTMIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TTMIL"]
    #[inline(always)]
    pub fn ttmil(&self) -> TTMIL_R {
        TTMIL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SWEL"]
    #[inline(always)]
    pub fn swel(&self) -> SWEL_R {
        SWEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GTWL"]
    #[inline(always)]
    pub fn gtwl(&self) -> GTWL_R {
        GTWL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GTDL"]
    #[inline(always)]
    pub fn gtdl(&self) -> GTDL_R {
        GTDL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTEL"]
    #[inline(always)]
    pub fn gtel(&self) -> GTEL_R {
        GTEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXUL"]
    #[inline(always)]
    pub fn txul(&self) -> TXUL_R {
        TXUL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXOL"]
    #[inline(always)]
    pub fn txol(&self) -> TXOL_R {
        TXOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SE1L"]
    #[inline(always)]
    pub fn se1l(&self) -> SE1L_R {
        SE1L_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SE2L"]
    #[inline(always)]
    pub fn se2l(&self) -> SE2L_R {
        SE2L_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ELCL"]
    #[inline(always)]
    pub fn elcl(&self) -> ELCL_R {
        ELCL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IWTL"]
    #[inline(always)]
    pub fn iwtl(&self) -> IWTL_R {
        IWTL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WTL"]
    #[inline(always)]
    pub fn wtl(&self) -> WTL_R {
        WTL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AWL"]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CERL"]
    #[inline(always)]
    pub fn cerl(&self) -> CERL_R {
        CERL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SBCL"]
    #[inline(always)]
    #[must_use]
    pub fn sbcl(&mut self) -> SBCL_W<FDCAN_TTILSrs> {
        SBCL_W::new(self, 0)
    }
    #[doc = "Bit 1 - SMCL"]
    #[inline(always)]
    #[must_use]
    pub fn smcl(&mut self) -> SMCL_W<FDCAN_TTILSrs> {
        SMCL_W::new(self, 1)
    }
    #[doc = "Bit 2 - CSML"]
    #[inline(always)]
    #[must_use]
    pub fn csml(&mut self) -> CSML_W<FDCAN_TTILSrs> {
        CSML_W::new(self, 2)
    }
    #[doc = "Bit 3 - SOGL"]
    #[inline(always)]
    #[must_use]
    pub fn sogl(&mut self) -> SOGL_W<FDCAN_TTILSrs> {
        SOGL_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTMIL"]
    #[inline(always)]
    #[must_use]
    pub fn rtmil(&mut self) -> RTMIL_W<FDCAN_TTILSrs> {
        RTMIL_W::new(self, 4)
    }
    #[doc = "Bit 5 - TTMIL"]
    #[inline(always)]
    #[must_use]
    pub fn ttmil(&mut self) -> TTMIL_W<FDCAN_TTILSrs> {
        TTMIL_W::new(self, 5)
    }
    #[doc = "Bit 6 - SWEL"]
    #[inline(always)]
    #[must_use]
    pub fn swel(&mut self) -> SWEL_W<FDCAN_TTILSrs> {
        SWEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - GTWL"]
    #[inline(always)]
    #[must_use]
    pub fn gtwl(&mut self) -> GTWL_W<FDCAN_TTILSrs> {
        GTWL_W::new(self, 7)
    }
    #[doc = "Bit 8 - GTDL"]
    #[inline(always)]
    #[must_use]
    pub fn gtdl(&mut self) -> GTDL_W<FDCAN_TTILSrs> {
        GTDL_W::new(self, 8)
    }
    #[doc = "Bit 9 - GTEL"]
    #[inline(always)]
    #[must_use]
    pub fn gtel(&mut self) -> GTEL_W<FDCAN_TTILSrs> {
        GTEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - TXUL"]
    #[inline(always)]
    #[must_use]
    pub fn txul(&mut self) -> TXUL_W<FDCAN_TTILSrs> {
        TXUL_W::new(self, 10)
    }
    #[doc = "Bit 11 - TXOL"]
    #[inline(always)]
    #[must_use]
    pub fn txol(&mut self) -> TXOL_W<FDCAN_TTILSrs> {
        TXOL_W::new(self, 11)
    }
    #[doc = "Bit 12 - SE1L"]
    #[inline(always)]
    #[must_use]
    pub fn se1l(&mut self) -> SE1L_W<FDCAN_TTILSrs> {
        SE1L_W::new(self, 12)
    }
    #[doc = "Bit 13 - SE2L"]
    #[inline(always)]
    #[must_use]
    pub fn se2l(&mut self) -> SE2L_W<FDCAN_TTILSrs> {
        SE2L_W::new(self, 13)
    }
    #[doc = "Bit 14 - ELCL"]
    #[inline(always)]
    #[must_use]
    pub fn elcl(&mut self) -> ELCL_W<FDCAN_TTILSrs> {
        ELCL_W::new(self, 14)
    }
    #[doc = "Bit 15 - IWTL"]
    #[inline(always)]
    #[must_use]
    pub fn iwtl(&mut self) -> IWTL_W<FDCAN_TTILSrs> {
        IWTL_W::new(self, 15)
    }
    #[doc = "Bit 16 - WTL"]
    #[inline(always)]
    #[must_use]
    pub fn wtl(&mut self) -> WTL_W<FDCAN_TTILSrs> {
        WTL_W::new(self, 16)
    }
    #[doc = "Bit 17 - AWL"]
    #[inline(always)]
    #[must_use]
    pub fn awl(&mut self) -> AWL_W<FDCAN_TTILSrs> {
        AWL_W::new(self, 17)
    }
    #[doc = "Bit 18 - CERL"]
    #[inline(always)]
    #[must_use]
    pub fn cerl(&mut self) -> CERL_W<FDCAN_TTILSrs> {
        CERL_W::new(self, 18)
    }
}
#[doc = "The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttils::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttils::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTILSrs;
impl crate::RegisterSpec for FDCAN_TTILSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttils::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTILSrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttils::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTILSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTILS to value 0"]
impl crate::Resettable for FDCAN_TTILSrs {
    const RESET_VALUE: u32 = 0;
}
