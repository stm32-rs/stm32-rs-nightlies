///Register `AGC3_CTRL` reader
pub type R = crate::R<AGC3_CTRLrs>;
///Register `AGC3_CTRL` writer
pub type W = crate::W<AGC3_CTRLrs>;
///Field `AGC_MIN_ATTEN` reader - Minimum AGC attenuation.
pub type AGC_MIN_ATTEN_R = crate::FieldReader;
///Field `AGC_MIN_ATTEN` writer - Minimum AGC attenuation.
pub type AGC_MIN_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AGC_MAX_ATTEN` reader - Maximum AGC attenuation.
pub type AGC_MAX_ATTEN_R = crate::FieldReader;
///Field `AGC_MAX_ATTEN` writer - Maximum AGC attenuation.
pub type AGC_MAX_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Minimum AGC attenuation.
    #[inline(always)]
    pub fn agc_min_atten(&self) -> AGC_MIN_ATTEN_R {
        AGC_MIN_ATTEN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Maximum AGC attenuation.
    #[inline(always)]
    pub fn agc_max_atten(&self) -> AGC_MAX_ATTEN_R {
        AGC_MAX_ATTEN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC3_CTRL")
            .field("agc_min_atten", &self.agc_min_atten())
            .field("agc_max_atten", &self.agc_max_atten())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Minimum AGC attenuation.
    #[inline(always)]
    pub fn agc_min_atten(&mut self) -> AGC_MIN_ATTEN_W<'_, AGC3_CTRLrs> {
        AGC_MIN_ATTEN_W::new(self, 0)
    }
    ///Bits 4:7 - Maximum AGC attenuation.
    #[inline(always)]
    pub fn agc_max_atten(&mut self) -> AGC_MAX_ATTEN_W<'_, AGC3_CTRLrs> {
        AGC_MAX_ATTEN_W::new(self, 4)
    }
}
/**AGC3_CTRL register

You can [`read`](crate::Reg::read) this register and get [`agc3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC3_CTRL)*/
pub struct AGC3_CTRLrs;
impl crate::RegisterSpec for AGC3_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`agc3_ctrl::R`](R) reader structure
impl crate::Readable for AGC3_CTRLrs {}
///`write(|w| ..)` method takes [`agc3_ctrl::W`](W) writer structure
impl crate::Writable for AGC3_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC3_CTRL to value 0x90
impl crate::Resettable for AGC3_CTRLrs {
    const RESET_VALUE: u32 = 0x90;
}
