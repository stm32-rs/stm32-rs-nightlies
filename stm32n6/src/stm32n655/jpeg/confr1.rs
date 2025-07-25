///Register `CONFR1` reader
pub type R = crate::R<CONFR1rs>;
///Register `CONFR1` writer
pub type W = crate::W<CONFR1rs>;
///Field `NF` reader - Number of color components
pub type NF_R = crate::FieldReader;
///Field `NF` writer - Number of color components
pub type NF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DE` reader - Codec operation as coder or decoder
pub type DE_R = crate::BitReader;
///Field `DE` writer - Codec operation as coder or decoder
pub type DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COLSPACE` reader - Color space
pub type COLSPACE_R = crate::FieldReader;
///Field `COLSPACE` writer - Color space
pub type COLSPACE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NS` reader - Number of components for scan
pub type NS_R = crate::FieldReader;
///Field `NS` writer - Number of components for scan
pub type NS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HDR` reader - Header processing
pub type HDR_R = crate::BitReader;
///Field `HDR` writer - Header processing
pub type HDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `YSIZE` reader - Y Size
pub type YSIZE_R = crate::FieldReader<u16>;
///Field `YSIZE` writer - Y Size
pub type YSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:1 - Number of color components
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - Codec operation as coder or decoder
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Color space
    #[inline(always)]
    pub fn colspace(&self) -> COLSPACE_R {
        COLSPACE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Number of components for scan
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - Header processing
    #[inline(always)]
    pub fn hdr(&self) -> HDR_R {
        HDR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:31 - Y Size
    #[inline(always)]
    pub fn ysize(&self) -> YSIZE_R {
        YSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFR1")
            .field("nf", &self.nf())
            .field("de", &self.de())
            .field("colspace", &self.colspace())
            .field("ns", &self.ns())
            .field("hdr", &self.hdr())
            .field("ysize", &self.ysize())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Number of color components
    #[inline(always)]
    pub fn nf(&mut self) -> NF_W<CONFR1rs> {
        NF_W::new(self, 0)
    }
    ///Bit 3 - Codec operation as coder or decoder
    #[inline(always)]
    pub fn de(&mut self) -> DE_W<CONFR1rs> {
        DE_W::new(self, 3)
    }
    ///Bits 4:5 - Color space
    #[inline(always)]
    pub fn colspace(&mut self) -> COLSPACE_W<CONFR1rs> {
        COLSPACE_W::new(self, 4)
    }
    ///Bits 6:7 - Number of components for scan
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W<CONFR1rs> {
        NS_W::new(self, 6)
    }
    ///Bit 8 - Header processing
    #[inline(always)]
    pub fn hdr(&mut self) -> HDR_W<CONFR1rs> {
        HDR_W::new(self, 8)
    }
    ///Bits 16:31 - Y Size
    #[inline(always)]
    pub fn ysize(&mut self) -> YSIZE_W<CONFR1rs> {
        YSIZE_W::new(self, 16)
    }
}
/**JPEG codec configuration register 1

You can [`read`](crate::Reg::read) this register and get [`confr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:CONFR1)*/
pub struct CONFR1rs;
impl crate::RegisterSpec for CONFR1rs {
    type Ux = u32;
}
///`read()` method returns [`confr1::R`](R) reader structure
impl crate::Readable for CONFR1rs {}
///`write(|w| ..)` method takes [`confr1::W`](W) writer structure
impl crate::Writable for CONFR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFR1 to value 0
impl crate::Resettable for CONFR1rs {}
