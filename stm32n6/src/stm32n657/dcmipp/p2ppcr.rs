///Register `P2PPCR` reader
pub type R = crate::R<P2PPCRrs>;
///Register `P2PPCR` writer
pub type W = crate::W<P2PPCRrs>;
///Field `FORMAT` reader - Memory format (only coplanar formats are supported in Pipe2)
pub type FORMAT_R = crate::FieldReader;
///Field `FORMAT` writer - Memory format (only coplanar formats are supported in Pipe2)
pub type FORMAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SWAPRB` reader - Swaps R-vs-B components if RGB, and if YUV, swaps U-vs-V components
pub type SWAPRB_R = crate::BitReader;
///Field `SWAPRB` writer - Swaps R-vs-B components if RGB, and if YUV, swaps U-vs-V components
pub type SWAPRB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINEMULT` reader - Amount of capture completed lines for LINE event and interrupt
pub type LINEMULT_R = crate::FieldReader;
///Field `LINEMULT` writer - Amount of capture completed lines for LINE event and interrupt
pub type LINEMULT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBM` reader - Double buffer mode
pub type DBM_R = crate::BitReader;
///Field `DBM` writer - Double buffer mode
pub type DBM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LMAWM` reader - Line multi address wrapping modulo
pub type LMAWM_R = crate::FieldReader;
///Field `LMAWM` writer - Line multi address wrapping modulo
pub type LMAWM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LMAWE` reader - Line multi address wrapping enable bit
pub type LMAWE_R = crate::BitReader;
///Field `LMAWE` writer - Line multi address wrapping enable bit
pub type LMAWE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Memory format (only coplanar formats are supported in Pipe2)
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Swaps R-vs-B components if RGB, and if YUV, swaps U-vs-V components
    #[inline(always)]
    pub fn swaprb(&self) -> SWAPRB_R {
        SWAPRB_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 13:15 - Amount of capture completed lines for LINE event and interrupt
    #[inline(always)]
    pub fn linemult(&self) -> LINEMULT_R {
        LINEMULT_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bit 16 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Line multi address wrapping modulo
    #[inline(always)]
    pub fn lmawm(&self) -> LMAWM_R {
        LMAWM_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - Line multi address wrapping enable bit
    #[inline(always)]
    pub fn lmawe(&self) -> LMAWE_R {
        LMAWE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2PPCR")
            .field("format", &self.format())
            .field("swaprb", &self.swaprb())
            .field("linemult", &self.linemult())
            .field("dbm", &self.dbm())
            .field("lmawm", &self.lmawm())
            .field("lmawe", &self.lmawe())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Memory format (only coplanar formats are supported in Pipe2)
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W<'_, P2PPCRrs> {
        FORMAT_W::new(self, 0)
    }
    ///Bit 4 - Swaps R-vs-B components if RGB, and if YUV, swaps U-vs-V components
    #[inline(always)]
    pub fn swaprb(&mut self) -> SWAPRB_W<'_, P2PPCRrs> {
        SWAPRB_W::new(self, 4)
    }
    ///Bits 13:15 - Amount of capture completed lines for LINE event and interrupt
    #[inline(always)]
    pub fn linemult(&mut self) -> LINEMULT_W<'_, P2PPCRrs> {
        LINEMULT_W::new(self, 13)
    }
    ///Bit 16 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W<'_, P2PPCRrs> {
        DBM_W::new(self, 16)
    }
    ///Bits 17:19 - Line multi address wrapping modulo
    #[inline(always)]
    pub fn lmawm(&mut self) -> LMAWM_W<'_, P2PPCRrs> {
        LMAWM_W::new(self, 17)
    }
    ///Bit 20 - Line multi address wrapping enable bit
    #[inline(always)]
    pub fn lmawe(&mut self) -> LMAWE_W<'_, P2PPCRrs> {
        LMAWE_W::new(self, 20)
    }
}
/**DCMIPP Pipe2 pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p2ppcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ppcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2PPCR)*/
pub struct P2PPCRrs;
impl crate::RegisterSpec for P2PPCRrs {
    type Ux = u32;
}
///`read()` method returns [`p2ppcr::R`](R) reader structure
impl crate::Readable for P2PPCRrs {}
///`write(|w| ..)` method takes [`p2ppcr::W`](W) writer structure
impl crate::Writable for P2PPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P2PPCR to value 0
impl crate::Resettable for P2PPCRrs {}
