///Register `BWTR2` reader
pub type R = crate::R<BWTR2rs>;
///Register `BWTR2` writer
pub type W = crate::W<BWTR2rs>;
///Field `ADDSET` reader - ADDSET
pub type ADDSET_R = crate::FieldReader;
///Field `ADDSET` writer - ADDSET
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDHLD` reader - ADDHLD
pub type ADDHLD_R = crate::FieldReader;
///Field `ADDHLD` writer - ADDHLD
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATAST` reader - DATAST
pub type DATAST_R = crate::FieldReader;
///Field `DATAST` writer - DATAST
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BUSTURN` reader - Bus turnaround phase duration
pub type BUSTURN_R = crate::FieldReader;
///Field `BUSTURN` writer - Bus turnaround phase duration
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CLKDIV` reader - CLKDIV
pub type CLKDIV_R = crate::FieldReader;
///Field `CLKDIV` writer - CLKDIV
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATLAT` reader - DATLAT
pub type DATLAT_R = crate::FieldReader;
///Field `DATLAT` writer - DATLAT
pub type DATLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ACCMOD` reader - ACCMOD
pub type ACCMOD_R = crate::FieldReader;
///Field `ACCMOD` writer - ACCMOD
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - ADDSET
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Bus turnaround phase duration
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - CLKDIV
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - DATLAT
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BWTR2")
            .field("accmod", &self.accmod())
            .field("datlat", &self.datlat())
            .field("clkdiv", &self.clkdiv())
            .field("busturn", &self.busturn())
            .field("datast", &self.datast())
            .field("addhld", &self.addhld())
            .field("addset", &self.addset())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ADDSET
    #[inline(always)]
    pub fn addset(&mut self) -> ADDSET_W<BWTR2rs> {
        ADDSET_W::new(self, 0)
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W<BWTR2rs> {
        ADDHLD_W::new(self, 4)
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W<BWTR2rs> {
        DATAST_W::new(self, 8)
    }
    ///Bits 16:19 - Bus turnaround phase duration
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W<BWTR2rs> {
        BUSTURN_W::new(self, 16)
    }
    ///Bits 20:23 - CLKDIV
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<BWTR2rs> {
        CLKDIV_W::new(self, 20)
    }
    ///Bits 24:27 - DATLAT
    #[inline(always)]
    pub fn datlat(&mut self) -> DATLAT_W<BWTR2rs> {
        DATLAT_W::new(self, 24)
    }
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W<BWTR2rs> {
        ACCMOD_W::new(self, 28)
    }
}
/**SRAM/NOR-Flash write timing registers 2

You can [`read`](crate::Reg::read) this register and get [`bwtr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#FMC:BWTR2)*/
pub struct BWTR2rs;
impl crate::RegisterSpec for BWTR2rs {
    type Ux = u32;
}
///`read()` method returns [`bwtr2::R`](R) reader structure
impl crate::Readable for BWTR2rs {}
///`write(|w| ..)` method takes [`bwtr2::W`](W) writer structure
impl crate::Writable for BWTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BWTR2 to value 0x0fff_ffff
impl crate::Resettable for BWTR2rs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
