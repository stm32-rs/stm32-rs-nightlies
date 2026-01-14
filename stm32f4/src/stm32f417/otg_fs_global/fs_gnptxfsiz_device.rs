///Register `FS_GNPTXFSIZ_Device` reader
pub type R = crate::R<FS_GNPTXFSIZ_DEVICErs>;
///Register `FS_GNPTXFSIZ_Device` writer
pub type W = crate::W<FS_GNPTXFSIZ_DEVICErs>;
///Field `TX0FSA` reader - Endpoint 0 transmit RAM start address
pub type TX0FSA_R = crate::FieldReader<u16>;
///Field `TX0FSA` writer - Endpoint 0 transmit RAM start address
pub type TX0FSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TX0FD` reader - Endpoint 0 TxFIFO depth
pub type TX0FD_R = crate::FieldReader<u16>;
///Field `TX0FD` writer - Endpoint 0 TxFIFO depth
pub type TX0FD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Endpoint 0 transmit RAM start address
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Endpoint 0 TxFIFO depth
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GNPTXFSIZ_Device")
            .field("tx0fsa", &self.tx0fsa())
            .field("tx0fd", &self.tx0fd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Endpoint 0 transmit RAM start address
    #[inline(always)]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<'_, FS_GNPTXFSIZ_DEVICErs> {
        TX0FSA_W::new(self, 0)
    }
    ///Bits 16:31 - Endpoint 0 TxFIFO depth
    #[inline(always)]
    pub fn tx0fd(&mut self) -> TX0FD_W<'_, FS_GNPTXFSIZ_DEVICErs> {
        TX0FD_W::new(self, 16)
    }
}
/**OTG_FS non-periodic transmit FIFO size register (Device mode)

You can [`read`](crate::Reg::read) this register and get [`fs_gnptxfsiz_device::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gnptxfsiz_device::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_FS_GLOBAL:FS_GNPTXFSIZ_Device)*/
pub struct FS_GNPTXFSIZ_DEVICErs;
impl crate::RegisterSpec for FS_GNPTXFSIZ_DEVICErs {
    type Ux = u32;
}
///`read()` method returns [`fs_gnptxfsiz_device::R`](R) reader structure
impl crate::Readable for FS_GNPTXFSIZ_DEVICErs {}
///`write(|w| ..)` method takes [`fs_gnptxfsiz_device::W`](W) writer structure
impl crate::Writable for FS_GNPTXFSIZ_DEVICErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_GNPTXFSIZ_Device to value 0x0200
impl crate::Resettable for FS_GNPTXFSIZ_DEVICErs {
    const RESET_VALUE: u32 = 0x0200;
}
