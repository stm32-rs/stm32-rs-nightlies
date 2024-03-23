#[doc = "Register `BTR3` reader"]
pub type R = crate::R<BTR3rs>;
#[doc = "Register `BTR3` writer"]
pub type W = crate::W<BTR3rs>;
#[doc = "Field `ADDSET` reader - Address setup phase duration"]
pub type ADDSET_R = crate::FieldReader;
#[doc = "Field `ADDSET` writer - Address setup phase duration"]
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDHLD` reader - Address-hold phase duration"]
pub type ADDHLD_R = crate::FieldReader;
#[doc = "Field `ADDHLD` writer - Address-hold phase duration"]
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - Data-phase duration"]
pub type DATAST_R = crate::FieldReader;
#[doc = "Field `DATAST` writer - Data-phase duration"]
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSTURN` reader - Bus turnaround phase duration"]
pub type BUSTURN_R = crate::FieldReader;
#[doc = "Field `BUSTURN` writer - Bus turnaround phase duration"]
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKDIV` reader - Clock divide ratio (for FMC_CLK signal)"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divide ratio (for FMC_CLK signal)"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATLAT` reader - Data latency for synchronous memory"]
pub type DATLAT_R = crate::FieldReader;
#[doc = "Field `DATLAT` writer - Data latency for synchronous memory"]
pub type DATLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACCMOD` reader - Access mode"]
pub type ACCMOD_R = crate::FieldReader;
#[doc = "Field `ACCMOD` writer - Access mode"]
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATAHLD` reader - Data hold phase duration"]
pub type DATAHLD_R = crate::FieldReader;
#[doc = "Field `DATAHLD` writer - Data hold phase duration"]
pub type DATAHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Address setup phase duration"]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration"]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data-phase duration"]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration"]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Clock divide ratio (for FMC_CLK signal)"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Data latency for synchronous memory"]
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Access mode"]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Data hold phase duration"]
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<BTR3rs> {
        ADDSET_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<BTR3rs> {
        ADDHLD_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Data-phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<BTR3rs> {
        DATAST_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<BTR3rs> {
        BUSTURN_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Clock divide ratio (for FMC_CLK signal)"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<BTR3rs> {
        CLKDIV_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Data latency for synchronous memory"]
    #[inline(always)]
    #[must_use]
    pub fn datlat(&mut self) -> DATLAT_W<BTR3rs> {
        DATLAT_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Access mode"]
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<BTR3rs> {
        ACCMOD_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Data hold phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn datahld(&mut self) -> DATAHLD_W<BTR3rs> {
        DATAHLD_W::new(self, 30)
    }
}
#[doc = "SRAM/NOR-Flash chip-select timing register for bank 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTR3rs;
impl crate::RegisterSpec for BTR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr3::R`](R) reader structure"]
impl crate::Readable for BTR3rs {}
#[doc = "`write(|w| ..)` method takes [`btr3::W`](W) writer structure"]
impl crate::Writable for BTR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTR3 to value 0x0fff_ffff"]
impl crate::Resettable for BTR3rs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
