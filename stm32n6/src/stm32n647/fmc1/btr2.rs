///Register `BTR2` reader
pub type R = crate::R<BTR2rs>;
///Register `BTR2` writer
pub type W = crate::W<BTR2rs>;
///Field `ADDSET` reader - Address setup phase duration
pub type ADDSET_R = crate::FieldReader;
///Field `ADDSET` writer - Address setup phase duration
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDHLD` reader - Address-hold phase duration
pub type ADDHLD_R = crate::FieldReader;
///Field `ADDHLD` writer - Address-hold phase duration
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATAST` reader - Data-phase duration
pub type DATAST_R = crate::FieldReader;
///Field `DATAST` writer - Data-phase duration
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BUSTURN` reader - Bus turnaround phase duration
pub type BUSTURN_R = crate::FieldReader;
///Field `BUSTURN` writer - Bus turnaround phase duration
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CLKDIV` reader - Clock divide ratio (for FMC_CLK signal)
pub type CLKDIV_R = crate::FieldReader;
///Field `CLKDIV` writer - Clock divide ratio (for FMC_CLK signal)
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATLAT` reader - Data latency for synchronous memory (see note below bit descriptions)
pub type DATLAT_R = crate::FieldReader;
///Field `DATLAT` writer - Data latency for synchronous memory (see note below bit descriptions)
pub type DATLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ACCMOD` reader - Access mode
pub type ACCMOD_R = crate::FieldReader;
///Field `ACCMOD` writer - Access mode
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DATAHLD` reader - Data Hold phase duration
pub type DATAHLD_R = crate::FieldReader;
///Field `DATAHLD` writer - Data Hold phase duration
pub type DATAHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - Address setup phase duration
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Address-hold phase duration
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - Data-phase duration
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Bus turnaround phase duration
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Clock divide ratio (for FMC_CLK signal)
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Data latency for synchronous memory (see note below bit descriptions)
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - Access mode
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Data Hold phase duration
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTR2")
            .field("addset", &self.addset())
            .field("addhld", &self.addhld())
            .field("datast", &self.datast())
            .field("busturn", &self.busturn())
            .field("clkdiv", &self.clkdiv())
            .field("datlat", &self.datlat())
            .field("accmod", &self.accmod())
            .field("datahld", &self.datahld())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Address setup phase duration
    #[inline(always)]
    pub fn addset(&mut self) -> ADDSET_W<'_, BTR2rs> {
        ADDSET_W::new(self, 0)
    }
    ///Bits 4:7 - Address-hold phase duration
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W<'_, BTR2rs> {
        ADDHLD_W::new(self, 4)
    }
    ///Bits 8:15 - Data-phase duration
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W<'_, BTR2rs> {
        DATAST_W::new(self, 8)
    }
    ///Bits 16:19 - Bus turnaround phase duration
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W<'_, BTR2rs> {
        BUSTURN_W::new(self, 16)
    }
    ///Bits 20:23 - Clock divide ratio (for FMC_CLK signal)
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<'_, BTR2rs> {
        CLKDIV_W::new(self, 20)
    }
    ///Bits 24:27 - Data latency for synchronous memory (see note below bit descriptions)
    #[inline(always)]
    pub fn datlat(&mut self) -> DATLAT_W<'_, BTR2rs> {
        DATLAT_W::new(self, 24)
    }
    ///Bits 28:29 - Access mode
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W<'_, BTR2rs> {
        ACCMOD_W::new(self, 28)
    }
    ///Bits 30:31 - Data Hold phase duration
    #[inline(always)]
    pub fn datahld(&mut self) -> DATAHLD_W<'_, BTR2rs> {
        DATAHLD_W::new(self, 30)
    }
}
/**SRAM/NOR Flash chip-select timing registers for memory region 2

You can [`read`](crate::Reg::read) this register and get [`btr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FMC1:BTR2)*/
pub struct BTR2rs;
impl crate::RegisterSpec for BTR2rs {
    type Ux = u32;
}
///`read()` method returns [`btr2::R`](R) reader structure
impl crate::Readable for BTR2rs {}
///`write(|w| ..)` method takes [`btr2::W`](W) writer structure
impl crate::Writable for BTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BTR2 to value 0x0fff_ffff
impl crate::Resettable for BTR2rs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
