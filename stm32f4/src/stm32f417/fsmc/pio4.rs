///Register `PIO4` reader
pub type R = crate::R<PIO4rs>;
///Register `PIO4` writer
pub type W = crate::W<PIO4rs>;
///Field `IOSETx` reader - IOSETx
pub type IOSETX_R = crate::FieldReader;
///Field `IOSETx` writer - IOSETx
pub type IOSETX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IOWAITx` reader - IOWAITx
pub type IOWAITX_R = crate::FieldReader;
///Field `IOWAITx` writer - IOWAITx
pub type IOWAITX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IOHOLDx` reader - IOHOLDx
pub type IOHOLDX_R = crate::FieldReader;
///Field `IOHOLDx` writer - IOHOLDx
pub type IOHOLDX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IOHIZx` reader - IOHIZx
pub type IOHIZX_R = crate::FieldReader;
///Field `IOHIZx` writer - IOHIZx
pub type IOHIZX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - IOSETx
    #[inline(always)]
    pub fn iosetx(&self) -> IOSETX_R {
        IOSETX_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - IOWAITx
    #[inline(always)]
    pub fn iowaitx(&self) -> IOWAITX_R {
        IOWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - IOHOLDx
    #[inline(always)]
    pub fn ioholdx(&self) -> IOHOLDX_R {
        IOHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - IOHIZx
    #[inline(always)]
    pub fn iohizx(&self) -> IOHIZX_R {
        IOHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO4")
            .field("iohizx", &self.iohizx())
            .field("ioholdx", &self.ioholdx())
            .field("iowaitx", &self.iowaitx())
            .field("iosetx", &self.iosetx())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - IOSETx
    #[inline(always)]
    pub fn iosetx(&mut self) -> IOSETX_W<'_, PIO4rs> {
        IOSETX_W::new(self, 0)
    }
    ///Bits 8:15 - IOWAITx
    #[inline(always)]
    pub fn iowaitx(&mut self) -> IOWAITX_W<'_, PIO4rs> {
        IOWAITX_W::new(self, 8)
    }
    ///Bits 16:23 - IOHOLDx
    #[inline(always)]
    pub fn ioholdx(&mut self) -> IOHOLDX_W<'_, PIO4rs> {
        IOHOLDX_W::new(self, 16)
    }
    ///Bits 24:31 - IOHIZx
    #[inline(always)]
    pub fn iohizx(&mut self) -> IOHIZX_W<'_, PIO4rs> {
        IOHIZX_W::new(self, 24)
    }
}
/**I/O space timing register 4

You can [`read`](crate::Reg::read) this register and get [`pio4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#FSMC:PIO4)*/
pub struct PIO4rs;
impl crate::RegisterSpec for PIO4rs {
    type Ux = u32;
}
///`read()` method returns [`pio4::R`](R) reader structure
impl crate::Readable for PIO4rs {}
///`write(|w| ..)` method takes [`pio4::W`](W) writer structure
impl crate::Writable for PIO4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIO4 to value 0xfcfc_fcfc
impl crate::Resettable for PIO4rs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
