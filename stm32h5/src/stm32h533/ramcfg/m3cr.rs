///Register `M3CR` reader
pub type R = crate::R<M3CRrs>;
///Register `M3CR` writer
pub type W = crate::W<M3CRrs>;
///Field `ECCE` reader - ECC enable.
pub type ECCE_R = crate::BitReader;
///Field `ECCE` writer - ECC enable.
pub type ECCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALE` reader - Address latch enable
pub type ALE_R = crate::BitReader;
///Field `ALE` writer - Address latch enable
pub type ALE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAMER` reader - SRAM erase
pub type SRAMER_R = crate::BitReader;
///Field `SRAMER` writer - SRAM erase
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ECC enable.
    #[inline(always)]
    pub fn ecce(&self) -> ECCE_R {
        ECCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Address latch enable
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3CR")
            .field("ecce", &self.ecce())
            .field("ale", &self.ale())
            .field("sramer", &self.sramer())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECC enable.
    #[inline(always)]
    pub fn ecce(&mut self) -> ECCE_W<'_, M3CRrs> {
        ECCE_W::new(self, 0)
    }
    ///Bit 4 - Address latch enable
    #[inline(always)]
    pub fn ale(&mut self) -> ALE_W<'_, M3CRrs> {
        ALE_W::new(self, 4)
    }
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&mut self) -> SRAMER_W<'_, M3CRrs> {
        SRAMER_W::new(self, 8)
    }
}
/**RAMCFG memory 3 control register

You can [`read`](crate::Reg::read) this register and get [`m3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RAMCFG:M3CR)*/
pub struct M3CRrs;
impl crate::RegisterSpec for M3CRrs {
    type Ux = u32;
}
///`read()` method returns [`m3cr::R`](R) reader structure
impl crate::Readable for M3CRrs {}
///`write(|w| ..)` method takes [`m3cr::W`](W) writer structure
impl crate::Writable for M3CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M3CR to value 0
impl crate::Resettable for M3CRrs {}
