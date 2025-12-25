///Register `SCSR` reader
pub type R = crate::R<SCSRrs>;
///Register `SCSR` writer
pub type W = crate::W<SCSRrs>;
///Field `CCMER` reader - CCM SRAM Erase
pub type CCMER_R = crate::BitReader;
///Field `CCMER` writer - CCM SRAM Erase
pub type CCMER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCMBSY` reader - CCM SRAM busy by erase operation
pub type CCMBSY_R = crate::BitReader;
impl R {
    ///Bit 0 - CCM SRAM Erase
    #[inline(always)]
    pub fn ccmer(&self) -> CCMER_R {
        CCMER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CCM SRAM busy by erase operation
    #[inline(always)]
    pub fn ccmbsy(&self) -> CCMBSY_R {
        CCMBSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCSR")
            .field("ccmer", &self.ccmer())
            .field("ccmbsy", &self.ccmbsy())
            .finish()
    }
}
impl W {
    ///Bit 0 - CCM SRAM Erase
    #[inline(always)]
    pub fn ccmer(&mut self) -> CCMER_W<'_, SCSRrs> {
        CCMER_W::new(self, 0)
    }
}
/**CCM SRAM control and status register

You can [`read`](crate::Reg::read) this register and get [`scsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#SYSCFG:SCSR)*/
pub struct SCSRrs;
impl crate::RegisterSpec for SCSRrs {
    type Ux = u32;
}
///`read()` method returns [`scsr::R`](R) reader structure
impl crate::Readable for SCSRrs {}
///`write(|w| ..)` method takes [`scsr::W`](W) writer structure
impl crate::Writable for SCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCSR to value 0
impl crate::Resettable for SCSRrs {}
