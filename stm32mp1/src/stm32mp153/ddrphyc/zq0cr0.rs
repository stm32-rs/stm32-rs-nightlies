///Register `ZQ0CR0` reader
pub type R = crate::R<ZQ0CR0rs>;
///Register `ZQ0CR0` writer
pub type W = crate::W<ZQ0CR0rs>;
///Field `ZDATA` reader - ZDATA
pub type ZDATA_R = crate::FieldReader<u32>;
///Field `ZDATA` writer - ZDATA
pub type ZDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `ZDEN` reader - ZDEN
pub type ZDEN_R = crate::BitReader;
///Field `ZDEN` writer - ZDEN
pub type ZDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZCALBYP` reader - ZCALBYP
pub type ZCALBYP_R = crate::BitReader;
///Field `ZCALBYP` writer - ZCALBYP
pub type ZCALBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZCAL` reader - ZCAL
pub type ZCAL_R = crate::BitReader;
///Field `ZCAL` writer - ZCAL
pub type ZCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZQPD` reader - ZQPD
pub type ZQPD_R = crate::BitReader;
///Field `ZQPD` writer - ZQPD
pub type ZQPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:19 - ZDATA
    #[inline(always)]
    pub fn zdata(&self) -> ZDATA_R {
        ZDATA_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 28 - ZDEN
    #[inline(always)]
    pub fn zden(&self) -> ZDEN_R {
        ZDEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ZCALBYP
    #[inline(always)]
    pub fn zcalbyp(&self) -> ZCALBYP_R {
        ZCALBYP_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ZCAL
    #[inline(always)]
    pub fn zcal(&self) -> ZCAL_R {
        ZCAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ZQPD
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZQ0CR0")
            .field("zdata", &self.zdata())
            .field("zden", &self.zden())
            .field("zcalbyp", &self.zcalbyp())
            .field("zcal", &self.zcal())
            .field("zqpd", &self.zqpd())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - ZDATA
    #[inline(always)]
    pub fn zdata(&mut self) -> ZDATA_W<'_, ZQ0CR0rs> {
        ZDATA_W::new(self, 0)
    }
    ///Bit 28 - ZDEN
    #[inline(always)]
    pub fn zden(&mut self) -> ZDEN_W<'_, ZQ0CR0rs> {
        ZDEN_W::new(self, 28)
    }
    ///Bit 29 - ZCALBYP
    #[inline(always)]
    pub fn zcalbyp(&mut self) -> ZCALBYP_W<'_, ZQ0CR0rs> {
        ZCALBYP_W::new(self, 29)
    }
    ///Bit 30 - ZCAL
    #[inline(always)]
    pub fn zcal(&mut self) -> ZCAL_W<'_, ZQ0CR0rs> {
        ZCAL_W::new(self, 30)
    }
    ///Bit 31 - ZQPD
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZQPD_W<'_, ZQ0CR0rs> {
        ZQPD_W::new(self, 31)
    }
}
/**DDRPHYC ZQ0C register 0

You can [`read`](crate::Reg::read) this register and get [`zq0cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zq0cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:ZQ0CR0)*/
pub struct ZQ0CR0rs;
impl crate::RegisterSpec for ZQ0CR0rs {
    type Ux = u32;
}
///`read()` method returns [`zq0cr0::R`](R) reader structure
impl crate::Readable for ZQ0CR0rs {}
///`write(|w| ..)` method takes [`zq0cr0::W`](W) writer structure
impl crate::Writable for ZQ0CR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ZQ0CR0 to value 0x014a
impl crate::Resettable for ZQ0CR0rs {
    const RESET_VALUE: u32 = 0x014a;
}
