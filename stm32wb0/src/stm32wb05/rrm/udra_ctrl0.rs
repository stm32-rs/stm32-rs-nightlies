///Register `UDRA_CTRL0` reader
pub type R = crate::R<UDRA_CTRL0rs>;
///Register `UDRA_CTRL0` writer
pub type W = crate::W<UDRA_CTRL0rs>;
///Field `RELOAD_RDCFGPTR` reader - reload the radio configuration pointer from RAM.
pub type RELOAD_RDCFGPTR_R = crate::BitReader;
///Field `RELOAD_RDCFGPTR` writer - reload the radio configuration pointer from RAM.
pub type RELOAD_RDCFGPTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - reload the radio configuration pointer from RAM.
    #[inline(always)]
    pub fn reload_rdcfgptr(&self) -> RELOAD_RDCFGPTR_R {
        RELOAD_RDCFGPTR_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDRA_CTRL0")
            .field("reload_rdcfgptr", &self.reload_rdcfgptr())
            .finish()
    }
}
impl W {
    ///Bit 0 - reload the radio configuration pointer from RAM.
    #[inline(always)]
    pub fn reload_rdcfgptr(&mut self) -> RELOAD_RDCFGPTR_W<'_, UDRA_CTRL0rs> {
        RELOAD_RDCFGPTR_W::new(self, 0)
    }
}
/**UDRA_CTRL0 register

You can [`read`](crate::Reg::read) this register and get [`udra_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udra_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RRM:UDRA_CTRL0)*/
pub struct UDRA_CTRL0rs;
impl crate::RegisterSpec for UDRA_CTRL0rs {
    type Ux = u32;
}
///`read()` method returns [`udra_ctrl0::R`](R) reader structure
impl crate::Readable for UDRA_CTRL0rs {}
///`write(|w| ..)` method takes [`udra_ctrl0::W`](W) writer structure
impl crate::Writable for UDRA_CTRL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UDRA_CTRL0 to value 0
impl crate::Resettable for UDRA_CTRL0rs {}
