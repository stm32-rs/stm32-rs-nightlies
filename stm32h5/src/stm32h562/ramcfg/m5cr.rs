///Register `M5CR` reader
pub type R = crate::R<M5CRrs>;
///Register `M5CR` writer
pub type W = crate::W<M5CRrs>;
///Field `ECCE` reader - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
pub type ECCE_R = crate::BitReader;
///Field `ECCE` writer - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
pub type ECCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALE` reader - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
pub type ALE_R = crate::BitReader;
///Field `ALE` writer - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
pub type ALE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAMER` reader - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.
pub type SRAMER_R = crate::BitReader;
///Field `SRAMER` writer - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
    #[inline(always)]
    pub fn ecce(&self) -> ECCE_R {
        ECCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5CR")
            .field("ecce", &self.ecce())
            .field("ale", &self.ale())
            .field("sramer", &self.sramer())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
    #[inline(always)]
    pub fn ecce(&mut self) -> ECCE_W<'_, M5CRrs> {
        ECCE_W::new(self, 0)
    }
    ///Bit 4 - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register.
    #[inline(always)]
    pub fn ale(&mut self) -> ALE_W<'_, M5CRrs> {
        ALE_W::new(self, 4)
    }
    ///Bit 8 - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation.
    #[inline(always)]
    pub fn sramer(&mut self) -> SRAMER_W<'_, M5CRrs> {
        SRAMER_W::new(self, 8)
    }
}
/**RAMCFG memory 5 control register

You can [`read`](crate::Reg::read) this register and get [`m5cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#RAMCFG:M5CR)*/
pub struct M5CRrs;
impl crate::RegisterSpec for M5CRrs {
    type Ux = u32;
}
///`read()` method returns [`m5cr::R`](R) reader structure
impl crate::Readable for M5CRrs {}
///`write(|w| ..)` method takes [`m5cr::W`](W) writer structure
impl crate::Writable for M5CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M5CR to value 0
impl crate::Resettable for M5CRrs {}
