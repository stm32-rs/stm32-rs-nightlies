///Register `SYSCFG_TSCCR` reader
pub type R = crate::R<SYSCFG_TSCCRrs>;
///Register `SYSCFG_TSCCR` writer
pub type W = crate::W<SYSCFG_TSCCRrs>;
///Field `G2_IO1` reader - Comparator mode for group 2 on I/O 1
pub type G2_IO1_R = crate::BitReader;
///Field `G2_IO1` writer - Comparator mode for group 2 on I/O 1
pub type G2_IO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G2_IO3` reader - Comparator mode for group 2 on I/O 3
pub type G2_IO3_R = crate::BitReader;
///Field `G2_IO3` writer - Comparator mode for group 2 on I/O 3
pub type G2_IO3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G4_IO3` reader - Comparator mode for group 4 on I/O 3
pub type G4_IO3_R = crate::BitReader;
///Field `G4_IO3` writer - Comparator mode for group 4 on I/O 3
pub type G4_IO3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G6_IO1` reader - Comparator mode for group 6 on I/O 1
pub type G6_IO1_R = crate::BitReader;
///Field `G6_IO1` writer - Comparator mode for group 6 on I/O 1
pub type G6_IO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G7_IO1` reader - Comparator mode for group 7 on I/O 1
pub type G7_IO1_R = crate::BitReader;
///Field `G7_IO1` writer - Comparator mode for group 7 on I/O 1
pub type G7_IO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSC_IOCTRL` reader - I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware.
pub type TSC_IOCTRL_R = crate::BitReader;
///Field `TSC_IOCTRL` writer - I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware.
pub type TSC_IOCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Comparator mode for group 2 on I/O 1
    #[inline(always)]
    pub fn g2_io1(&self) -> G2_IO1_R {
        G2_IO1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Comparator mode for group 2 on I/O 3
    #[inline(always)]
    pub fn g2_io3(&self) -> G2_IO3_R {
        G2_IO3_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Comparator mode for group 4 on I/O 3
    #[inline(always)]
    pub fn g4_io3(&self) -> G4_IO3_R {
        G4_IO3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Comparator mode for group 6 on I/O 1
    #[inline(always)]
    pub fn g6_io1(&self) -> G6_IO1_R {
        G6_IO1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Comparator mode for group 7 on I/O 1
    #[inline(always)]
    pub fn g7_io1(&self) -> G7_IO1_R {
        G7_IO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware.
    #[inline(always)]
    pub fn tsc_ioctrl(&self) -> TSC_IOCTRL_R {
        TSC_IOCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_TSCCR")
            .field("g2_io1", &self.g2_io1())
            .field("g2_io3", &self.g2_io3())
            .field("g4_io3", &self.g4_io3())
            .field("g6_io1", &self.g6_io1())
            .field("g7_io1", &self.g7_io1())
            .field("tsc_ioctrl", &self.tsc_ioctrl())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator mode for group 2 on I/O 1
    #[inline(always)]
    pub fn g2_io1(&mut self) -> G2_IO1_W<SYSCFG_TSCCRrs> {
        G2_IO1_W::new(self, 0)
    }
    ///Bit 1 - Comparator mode for group 2 on I/O 3
    #[inline(always)]
    pub fn g2_io3(&mut self) -> G2_IO3_W<SYSCFG_TSCCRrs> {
        G2_IO3_W::new(self, 1)
    }
    ///Bit 2 - Comparator mode for group 4 on I/O 3
    #[inline(always)]
    pub fn g4_io3(&mut self) -> G4_IO3_W<SYSCFG_TSCCRrs> {
        G4_IO3_W::new(self, 2)
    }
    ///Bit 3 - Comparator mode for group 6 on I/O 1
    #[inline(always)]
    pub fn g6_io1(&mut self) -> G6_IO1_W<SYSCFG_TSCCRrs> {
        G6_IO1_W::new(self, 3)
    }
    ///Bit 4 - Comparator mode for group 7 on I/O 1
    #[inline(always)]
    pub fn g7_io1(&mut self) -> G7_IO1_W<SYSCFG_TSCCRrs> {
        G7_IO1_W::new(self, 4)
    }
    ///Bit 5 - I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware.
    #[inline(always)]
    pub fn tsc_ioctrl(&mut self) -> TSC_IOCTRL_W<SYSCFG_TSCCRrs> {
        TSC_IOCTRL_W::new(self, 5)
    }
}
/**SYSCFG TSC comparator register

You can [`read`](crate::Reg::read) this register and get [`syscfg_tsccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_tsccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SYSCFG:SYSCFG_TSCCR)*/
pub struct SYSCFG_TSCCRrs;
impl crate::RegisterSpec for SYSCFG_TSCCRrs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_tsccr::R`](R) reader structure
impl crate::Readable for SYSCFG_TSCCRrs {}
///`write(|w| ..)` method takes [`syscfg_tsccr::W`](W) writer structure
impl crate::Writable for SYSCFG_TSCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYSCFG_TSCCR to value 0
impl crate::Resettable for SYSCFG_TSCCRrs {
    const RESET_VALUE: u32 = 0;
}
