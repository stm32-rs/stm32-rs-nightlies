///Register `MP_IWDGFZSETR` reader
pub type R = crate::R<MP_IWDGFZSETRrs>;
///Register `MP_IWDGFZSETR` writer
pub type W = crate::W<MP_IWDGFZSETRrs>;
///Field `FZ_IWDG1` reader - FZ_IWDG1
pub type FZ_IWDG1_R = crate::BitReader;
///Field `FZ_IWDG1` writer - FZ_IWDG1
pub type FZ_IWDG1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FZ_IWDG2` reader - FZ_IWDG2
pub type FZ_IWDG2_R = crate::BitReader;
///Field `FZ_IWDG2` writer - FZ_IWDG2
pub type FZ_IWDG2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FZ_IWDG1
    #[inline(always)]
    pub fn fz_iwdg1(&self) -> FZ_IWDG1_R {
        FZ_IWDG1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FZ_IWDG2
    #[inline(always)]
    pub fn fz_iwdg2(&self) -> FZ_IWDG2_R {
        FZ_IWDG2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_IWDGFZSETR")
            .field("fz_iwdg1", &self.fz_iwdg1())
            .field("fz_iwdg2", &self.fz_iwdg2())
            .finish()
    }
}
impl W {
    ///Bit 0 - FZ_IWDG1
    #[inline(always)]
    pub fn fz_iwdg1(&mut self) -> FZ_IWDG1_W<'_, MP_IWDGFZSETRrs> {
        FZ_IWDG1_W::new(self, 0)
    }
    ///Bit 1 - FZ_IWDG2
    #[inline(always)]
    pub fn fz_iwdg2(&mut self) -> FZ_IWDG2_W<'_, MP_IWDGFZSETRrs> {
        FZ_IWDG2_W::new(self, 1)
    }
}
/**This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_iwdgfzsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_iwdgfzsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_IWDGFZSETR)*/
pub struct MP_IWDGFZSETRrs;
impl crate::RegisterSpec for MP_IWDGFZSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_iwdgfzsetr::R`](R) reader structure
impl crate::Readable for MP_IWDGFZSETRrs {}
///`write(|w| ..)` method takes [`mp_iwdgfzsetr::W`](W) writer structure
impl crate::Writable for MP_IWDGFZSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_IWDGFZSETR to value 0
impl crate::Resettable for MP_IWDGFZSETRrs {}
