///Register `AHB1SECSR` reader
pub type R = crate::R<AHB1SECSRrs>;
///Field `DMA1SECF` reader - DMA1SECF
pub type DMA1SECF_R = crate::BitReader;
///Field `DMA2SECF` reader - DMA2SECF
pub type DMA2SECF_R = crate::BitReader;
///Field `DMAMUX1SECF` reader - DMAMUX1SECF
pub type DMAMUX1SECF_R = crate::BitReader;
///Field `FLASHSECF` reader - FLASHSECF
pub type FLASHSECF_R = crate::BitReader;
///Field `SRAM1SECF` reader - SRAM1SECF
pub type SRAM1SECF_R = crate::BitReader;
///Field `CRCSECF` reader - CRCSECF
pub type CRCSECF_R = crate::BitReader;
///Field `TSCSECF` reader - TSCSECF
pub type TSCSECF_R = crate::BitReader;
///Field `GTZCSECF` reader - GTZCSECF
pub type GTZCSECF_R = crate::BitReader;
///Field `ICACHESECF` reader - ICACHESECF
pub type ICACHESECF_R = crate::BitReader;
impl R {
    ///Bit 0 - DMA1SECF
    #[inline(always)]
    pub fn dma1secf(&self) -> DMA1SECF_R {
        DMA1SECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2SECF
    #[inline(always)]
    pub fn dma2secf(&self) -> DMA2SECF_R {
        DMA2SECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUX1SECF
    #[inline(always)]
    pub fn dmamux1secf(&self) -> DMAMUX1SECF_R {
        DMAMUX1SECF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - FLASHSECF
    #[inline(always)]
    pub fn flashsecf(&self) -> FLASHSECF_R {
        FLASHSECF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM1SECF
    #[inline(always)]
    pub fn sram1secf(&self) -> SRAM1SECF_R {
        SRAM1SECF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRCSECF
    #[inline(always)]
    pub fn crcsecf(&self) -> CRCSECF_R {
        CRCSECF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - TSCSECF
    #[inline(always)]
    pub fn tscsecf(&self) -> TSCSECF_R {
        TSCSECF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 22 - GTZCSECF
    #[inline(always)]
    pub fn gtzcsecf(&self) -> GTZCSECF_R {
        GTZCSECF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ICACHESECF
    #[inline(always)]
    pub fn icachesecf(&self) -> ICACHESECF_R {
        ICACHESECF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1SECSR")
            .field("icachesecf", &self.icachesecf())
            .field("gtzcsecf", &self.gtzcsecf())
            .field("tscsecf", &self.tscsecf())
            .field("crcsecf", &self.crcsecf())
            .field("sram1secf", &self.sram1secf())
            .field("flashsecf", &self.flashsecf())
            .field("dmamux1secf", &self.dmamux1secf())
            .field("dma2secf", &self.dma2secf())
            .field("dma1secf", &self.dma1secf())
            .finish()
    }
}
/**RCC AHB1 security status register

You can [`read`](crate::Reg::read) this register and get [`ahb1secsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#RCC:AHB1SECSR)*/
pub struct AHB1SECSRrs;
impl crate::RegisterSpec for AHB1SECSRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1secsr::R`](R) reader structure
impl crate::Readable for AHB1SECSRrs {}
///`reset()` method sets AHB1SECSR to value 0x0040_0300
impl crate::Resettable for AHB1SECSRrs {
    const RESET_VALUE: u32 = 0x0040_0300;
}
