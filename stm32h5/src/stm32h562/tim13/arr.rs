///Register `ARR` reader
pub type R = crate::R<ARRrs>;
///Register `ARR` writer
pub type W = crate::W<ARRrs>;
/**Field `ARR` reader - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\[31:4\]. The ARR\[3:0\]
bitfield contains the dithered part.*/
pub type ARR_R = crate::FieldReader<u32>;
/**Field `ARR` writer - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\[31:4\]. The ARR\[3:0\]
bitfield contains the dithered part.*/
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `DITHER` reader - Dithered part in dithering mode
pub type DITHER_R = crate::FieldReader;
///Field `DITHER` writer - Dithered part in dithering mode
pub type DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `INTEGER` reader - Integer part in dithering mode
pub type INTEGER_R = crate::FieldReader<u16>;
///Field `INTEGER` writer - Integer part in dithering mode
pub type INTEGER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    /**Bits 0:19 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\[31:4\]. The ARR\[3:0\]
    bitfield contains the dithered part.*/
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits & 0x000f_ffff)
    }
    ///Bits 0:3 - Dithered part in dithering mode
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:19 - Integer part in dithering mode
    #[inline(always)]
    pub fn integer(&self) -> INTEGER_R {
        INTEGER_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARR")
            .field("arr", &self.arr())
            .field("integer", &self.integer())
            .field("dither", &self.dither())
            .finish()
    }
}
impl W {
    /**Bits 0:19 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\[31:4\]. The ARR\[3:0\]
    bitfield contains the dithered part.*/
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<ARRrs> {
        ARR_W::new(self, 0)
    }
    ///Bits 0:3 - Dithered part in dithering mode
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W<ARRrs> {
        DITHER_W::new(self, 0)
    }
    ///Bits 4:19 - Integer part in dithering mode
    #[inline(always)]
    pub fn integer(&mut self) -> INTEGER_W<ARRrs> {
        INTEGER_W::new(self, 4)
    }
}
/**TIM2 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#TIM13:ARR)*/
pub struct ARRrs;
impl crate::RegisterSpec for ARRrs {
    type Ux = u32;
}
///`read()` method returns [`arr::R`](R) reader structure
impl crate::Readable for ARRrs {}
///`write(|w| ..)` method takes [`arr::W`](W) writer structure
impl crate::Writable for ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ARR to value 0xffff_ffff
impl crate::Resettable for ARRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
