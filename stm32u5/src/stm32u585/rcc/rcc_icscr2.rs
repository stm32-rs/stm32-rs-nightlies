#[doc = "Register `RCC_ICSCR2` reader"]
pub type R = crate::R<RCC_ICSCR2rs>;
#[doc = "Register `RCC_ICSCR2` writer"]
pub type W = crate::W<RCC_ICSCR2rs>;
#[doc = "Field `MSITRIM3` reader - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
pub type MSITRIM3_R = crate::FieldReader;
#[doc = "Field `MSITRIM3` writer - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
pub type MSITRIM3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MSITRIM2` reader - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
pub type MSITRIM2_R = crate::FieldReader;
#[doc = "Field `MSITRIM2` writer - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
pub type MSITRIM2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MSITRIM1` reader - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
pub type MSITRIM1_R = crate::FieldReader;
#[doc = "Field `MSITRIM1` writer - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
pub type MSITRIM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MSITRIM0` reader - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
pub type MSITRIM0_R = crate::FieldReader;
#[doc = "Field `MSITRIM0` writer - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
pub type MSITRIM0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
    #[inline(always)]
    pub fn msitrim3(&self) -> MSITRIM3_R {
        MSITRIM3_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
    #[inline(always)]
    pub fn msitrim2(&self) -> MSITRIM2_R {
        MSITRIM2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
    #[inline(always)]
    pub fn msitrim1(&self) -> MSITRIM1_R {
        MSITRIM1_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
    #[inline(always)]
    pub fn msitrim0(&self) -> MSITRIM0_R {
        MSITRIM0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
    #[inline(always)]
    #[must_use]
    pub fn msitrim3(&mut self) -> MSITRIM3_W<RCC_ICSCR2rs> {
        MSITRIM3_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
    #[inline(always)]
    #[must_use]
    pub fn msitrim2(&mut self) -> MSITRIM2_W<RCC_ICSCR2rs> {
        MSITRIM2_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
    #[inline(always)]
    #[must_use]
    pub fn msitrim1(&mut self) -> MSITRIM1_W<RCC_ICSCR2rs> {
        MSITRIM1_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
    #[inline(always)]
    #[must_use]
    pub fn msitrim0(&mut self) -> MSITRIM0_W<RCC_ICSCR2rs> {
        MSITRIM0_W::new(self, 15)
    }
}
#[doc = "RCC internal clock sources calibration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_icscr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_icscr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_ICSCR2rs;
impl crate::RegisterSpec for RCC_ICSCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_icscr2::R`](R) reader structure"]
impl crate::Readable for RCC_ICSCR2rs {}
#[doc = "`write(|w| ..)` method takes [`rcc_icscr2::W`](W) writer structure"]
impl crate::Writable for RCC_ICSCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_ICSCR2 to value 0x0008_4210"]
impl crate::Resettable for RCC_ICSCR2rs {
    const RESET_VALUE: u32 = 0x0008_4210;
}
