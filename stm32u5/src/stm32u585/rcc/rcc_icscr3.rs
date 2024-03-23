#[doc = "Register `RCC_ICSCR3` reader"]
pub type R = crate::R<RCC_ICSCR3rs>;
#[doc = "Register `RCC_ICSCR3` writer"]
pub type W = crate::W<RCC_ICSCR3rs>;
#[doc = "Field `HSICAL` reader - HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
pub type HSICAL_R = crate::FieldReader<u16>;
#[doc = "Field `HSITRIM` reader - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to HSICAL\\[11:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI."]
pub type HSITRIM_R = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to HSICAL\\[11:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI."]
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:11 - HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:20 - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to HSICAL\\[11:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI."]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to HSICAL\\[11:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI."]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<RCC_ICSCR3rs> {
        HSITRIM_W::new(self, 16)
    }
}
#[doc = "RCC internal clock sources calibration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_icscr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_icscr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_ICSCR3rs;
impl crate::RegisterSpec for RCC_ICSCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_icscr3::R`](R) reader structure"]
impl crate::Readable for RCC_ICSCR3rs {}
#[doc = "`write(|w| ..)` method takes [`rcc_icscr3::W`](W) writer structure"]
impl crate::Writable for RCC_ICSCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_ICSCR3 to value 0x0010_0000"]
impl crate::Resettable for RCC_ICSCR3rs {
    const RESET_VALUE: u32 = 0x0010_0000;
}
