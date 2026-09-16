///Register `TTILS` reader
pub type R = crate::R<TTILSrs>;
///Register `TTILS` writer
pub type W = crate::W<TTILSrs>;
///Field `SBCL` reader - SBCL
pub type SBCL_R = crate::BitReader;
///Field `SBCL` writer - SBCL
pub type SBCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMCL` reader - SMCL
pub type SMCL_R = crate::BitReader;
///Field `SMCL` writer - SMCL
pub type SMCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSML` reader - CSML
pub type CSML_R = crate::BitReader;
///Field `CSML` writer - CSML
pub type CSML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOGL` reader - SOGL
pub type SOGL_R = crate::BitReader;
///Field `SOGL` writer - SOGL
pub type SOGL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTMIL` reader - RTMIL
pub type RTMIL_R = crate::BitReader;
///Field `RTMIL` writer - RTMIL
pub type RTMIL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TTMIL` reader - TTMIL
pub type TTMIL_R = crate::BitReader;
///Field `TTMIL` writer - TTMIL
pub type TTMIL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWEL` reader - SWEL
pub type SWEL_R = crate::BitReader;
///Field `SWEL` writer - SWEL
pub type SWEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTWL` reader - GTWL
pub type GTWL_R = crate::BitReader;
///Field `GTWL` writer - GTWL
pub type GTWL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTDL` reader - GTDL
pub type GTDL_R = crate::BitReader;
///Field `GTDL` writer - GTDL
pub type GTDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTEL` reader - GTEL
pub type GTEL_R = crate::BitReader;
///Field `GTEL` writer - GTEL
pub type GTEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUL` reader - TXUL
pub type TXUL_R = crate::BitReader;
///Field `TXUL` writer - TXUL
pub type TXUL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOL` reader - TXOL
pub type TXOL_R = crate::BitReader;
///Field `TXOL` writer - TXOL
pub type TXOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SE1L` reader - SE1L
pub type SE1L_R = crate::BitReader;
///Field `SE1L` writer - SE1L
pub type SE1L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SE2L` reader - SE2L
pub type SE2L_R = crate::BitReader;
///Field `SE2L` writer - SE2L
pub type SE2L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELCL` reader - ELCL
pub type ELCL_R = crate::BitReader;
///Field `ELCL` writer - ELCL
pub type ELCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWTL` reader - IWTL
pub type IWTL_R = crate::BitReader;
///Field `IWTL` writer - IWTL
pub type IWTL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WTL` reader - WTL
pub type WTL_R = crate::BitReader;
///Field `WTL` writer - WTL
pub type WTL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWL` reader - AWL
pub type AWL_R = crate::BitReader;
///Field `AWL` writer - AWL
pub type AWL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CERL` reader - CERL
pub type CERL_R = crate::BitReader;
///Field `CERL` writer - CERL
pub type CERL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SBCL
    #[inline(always)]
    pub fn sbcl(&self) -> SBCL_R {
        SBCL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SMCL
    #[inline(always)]
    pub fn smcl(&self) -> SMCL_R {
        SMCL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CSML
    #[inline(always)]
    pub fn csml(&self) -> CSML_R {
        CSML_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SOGL
    #[inline(always)]
    pub fn sogl(&self) -> SOGL_R {
        SOGL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RTMIL
    #[inline(always)]
    pub fn rtmil(&self) -> RTMIL_R {
        RTMIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TTMIL
    #[inline(always)]
    pub fn ttmil(&self) -> TTMIL_R {
        TTMIL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SWEL
    #[inline(always)]
    pub fn swel(&self) -> SWEL_R {
        SWEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTWL
    #[inline(always)]
    pub fn gtwl(&self) -> GTWL_R {
        GTWL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTDL
    #[inline(always)]
    pub fn gtdl(&self) -> GTDL_R {
        GTDL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTEL
    #[inline(always)]
    pub fn gtel(&self) -> GTEL_R {
        GTEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TXUL
    #[inline(always)]
    pub fn txul(&self) -> TXUL_R {
        TXUL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TXOL
    #[inline(always)]
    pub fn txol(&self) -> TXOL_R {
        TXOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SE1L
    #[inline(always)]
    pub fn se1l(&self) -> SE1L_R {
        SE1L_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SE2L
    #[inline(always)]
    pub fn se2l(&self) -> SE2L_R {
        SE2L_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ELCL
    #[inline(always)]
    pub fn elcl(&self) -> ELCL_R {
        ELCL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - IWTL
    #[inline(always)]
    pub fn iwtl(&self) -> IWTL_R {
        IWTL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - WTL
    #[inline(always)]
    pub fn wtl(&self) -> WTL_R {
        WTL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AWL
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CERL
    #[inline(always)]
    pub fn cerl(&self) -> CERL_R {
        CERL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTILS")
            .field("sbcl", &self.sbcl())
            .field("smcl", &self.smcl())
            .field("csml", &self.csml())
            .field("sogl", &self.sogl())
            .field("rtmil", &self.rtmil())
            .field("ttmil", &self.ttmil())
            .field("swel", &self.swel())
            .field("gtwl", &self.gtwl())
            .field("gtdl", &self.gtdl())
            .field("gtel", &self.gtel())
            .field("txul", &self.txul())
            .field("txol", &self.txol())
            .field("se1l", &self.se1l())
            .field("se2l", &self.se2l())
            .field("elcl", &self.elcl())
            .field("iwtl", &self.iwtl())
            .field("wtl", &self.wtl())
            .field("awl", &self.awl())
            .field("cerl", &self.cerl())
            .finish()
    }
}
impl W {
    ///Bit 0 - SBCL
    #[inline(always)]
    pub fn sbcl(&mut self) -> SBCL_W<'_, TTILSrs> {
        SBCL_W::new(self, 0)
    }
    ///Bit 1 - SMCL
    #[inline(always)]
    pub fn smcl(&mut self) -> SMCL_W<'_, TTILSrs> {
        SMCL_W::new(self, 1)
    }
    ///Bit 2 - CSML
    #[inline(always)]
    pub fn csml(&mut self) -> CSML_W<'_, TTILSrs> {
        CSML_W::new(self, 2)
    }
    ///Bit 3 - SOGL
    #[inline(always)]
    pub fn sogl(&mut self) -> SOGL_W<'_, TTILSrs> {
        SOGL_W::new(self, 3)
    }
    ///Bit 4 - RTMIL
    #[inline(always)]
    pub fn rtmil(&mut self) -> RTMIL_W<'_, TTILSrs> {
        RTMIL_W::new(self, 4)
    }
    ///Bit 5 - TTMIL
    #[inline(always)]
    pub fn ttmil(&mut self) -> TTMIL_W<'_, TTILSrs> {
        TTMIL_W::new(self, 5)
    }
    ///Bit 6 - SWEL
    #[inline(always)]
    pub fn swel(&mut self) -> SWEL_W<'_, TTILSrs> {
        SWEL_W::new(self, 6)
    }
    ///Bit 7 - GTWL
    #[inline(always)]
    pub fn gtwl(&mut self) -> GTWL_W<'_, TTILSrs> {
        GTWL_W::new(self, 7)
    }
    ///Bit 8 - GTDL
    #[inline(always)]
    pub fn gtdl(&mut self) -> GTDL_W<'_, TTILSrs> {
        GTDL_W::new(self, 8)
    }
    ///Bit 9 - GTEL
    #[inline(always)]
    pub fn gtel(&mut self) -> GTEL_W<'_, TTILSrs> {
        GTEL_W::new(self, 9)
    }
    ///Bit 10 - TXUL
    #[inline(always)]
    pub fn txul(&mut self) -> TXUL_W<'_, TTILSrs> {
        TXUL_W::new(self, 10)
    }
    ///Bit 11 - TXOL
    #[inline(always)]
    pub fn txol(&mut self) -> TXOL_W<'_, TTILSrs> {
        TXOL_W::new(self, 11)
    }
    ///Bit 12 - SE1L
    #[inline(always)]
    pub fn se1l(&mut self) -> SE1L_W<'_, TTILSrs> {
        SE1L_W::new(self, 12)
    }
    ///Bit 13 - SE2L
    #[inline(always)]
    pub fn se2l(&mut self) -> SE2L_W<'_, TTILSrs> {
        SE2L_W::new(self, 13)
    }
    ///Bit 14 - ELCL
    #[inline(always)]
    pub fn elcl(&mut self) -> ELCL_W<'_, TTILSrs> {
        ELCL_W::new(self, 14)
    }
    ///Bit 15 - IWTL
    #[inline(always)]
    pub fn iwtl(&mut self) -> IWTL_W<'_, TTILSrs> {
        IWTL_W::new(self, 15)
    }
    ///Bit 16 - WTL
    #[inline(always)]
    pub fn wtl(&mut self) -> WTL_W<'_, TTILSrs> {
        WTL_W::new(self, 16)
    }
    ///Bit 17 - AWL
    #[inline(always)]
    pub fn awl(&mut self) -> AWL_W<'_, TTILSrs> {
        AWL_W::new(self, 17)
    }
    ///Bit 18 - CERL
    #[inline(always)]
    pub fn cerl(&mut self) -> CERL_W<'_, TTILSrs> {
        CERL_W::new(self, 18)
    }
}
/**The TT interrupt Line select register assigns an interrupt generated by a specific interrupt flag from the TT interrupt register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via FDCAN_ILE.EINT0 and FDCAN_ILE.EINT1.

You can [`read`](crate::Reg::read) this register and get [`ttils::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttils::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTILS)*/
pub struct TTILSrs;
impl crate::RegisterSpec for TTILSrs {
    type Ux = u32;
}
///`read()` method returns [`ttils::R`](R) reader structure
impl crate::Readable for TTILSrs {}
///`write(|w| ..)` method takes [`ttils::W`](W) writer structure
impl crate::Writable for TTILSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTILS to value 0
impl crate::Resettable for TTILSrs {}
