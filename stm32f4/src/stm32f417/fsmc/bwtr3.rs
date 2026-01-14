///Register `BWTR3` reader
pub type R = crate::R<BWTR3rs>;
///Register `BWTR3` writer
pub type W = crate::W<BWTR3rs>;
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
///Field `BUSTURN` reader - BUSTURN
pub type BUSTURN_R = crate::FieldReader;
///Field `BUSTURN` writer - BUSTURN
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    ///Bits 16:19 - BUSTURN
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BWTR3")
            .field("accmod", &self.accmod())
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
    pub fn addset(&mut self) -> ADDSET_W<'_, BWTR3rs> {
        ADDSET_W::new(self, 0)
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W<'_, BWTR3rs> {
        ADDHLD_W::new(self, 4)
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W<'_, BWTR3rs> {
        DATAST_W::new(self, 8)
    }
    ///Bits 16:19 - BUSTURN
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W<'_, BWTR3rs> {
        BUSTURN_W::new(self, 16)
    }
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W<'_, BWTR3rs> {
        ACCMOD_W::new(self, 28)
    }
}
/**SRAM/NOR-Flash write timing registers 3

You can [`read`](crate::Reg::read) this register and get [`bwtr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#FSMC:BWTR3)*/
pub struct BWTR3rs;
impl crate::RegisterSpec for BWTR3rs {
    type Ux = u32;
}
///`read()` method returns [`bwtr3::R`](R) reader structure
impl crate::Readable for BWTR3rs {}
///`write(|w| ..)` method takes [`bwtr3::W`](W) writer structure
impl crate::Writable for BWTR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BWTR3 to value 0x0fff_ffff
impl crate::Resettable for BWTR3rs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
