///Register `OLD0CR` reader
pub type R = crate::R<OLD0CRrs>;
///Register `OLD0CR` writer
pub type W = crate::W<OLD0CRrs>;
///Field `OLDEN` reader - OLDx enable
pub type OLDEN_R = crate::BitReader;
///Field `OLDEN` writer - OLDx enable
pub type OLDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THINB` reader - Threshold In band
pub type THINB_R = crate::BitReader;
///Field `THINB` writer - Threshold In band
pub type THINB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKOLD` reader - Break signal assignment for out-of limit detector
pub type BKOLD_R = crate::FieldReader;
///Field `BKOLD` writer - Break signal assignment for out-of limit detector
pub type BKOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ACICN` reader - OLDx CIC order selection
pub type ACICN_R = crate::FieldReader;
///Field `ACICN` writer - OLDx CIC order selection
pub type ACICN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ACICD` reader - OLDx CIC decimation ratio selection
pub type ACICD_R = crate::FieldReader;
///Field `ACICD` writer - OLDx CIC decimation ratio selection
pub type ACICD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `OLDACTIVE` reader - OLDx active flag
pub type OLDACTIVE_R = crate::BitReader;
impl R {
    ///Bit 0 - OLDx enable
    #[inline(always)]
    pub fn olden(&self) -> OLDEN_R {
        OLDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Threshold In band
    #[inline(always)]
    pub fn thinb(&self) -> THINB_R {
        THINB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:7 - Break signal assignment for out-of limit detector
    #[inline(always)]
    pub fn bkold(&self) -> BKOLD_R {
        BKOLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 12:13 - OLDx CIC order selection
    #[inline(always)]
    pub fn acicn(&self) -> ACICN_R {
        ACICN_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 17:21 - OLDx CIC decimation ratio selection
    #[inline(always)]
    pub fn acicd(&self) -> ACICD_R {
        ACICD_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 31 - OLDx active flag
    #[inline(always)]
    pub fn oldactive(&self) -> OLDACTIVE_R {
        OLDACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OLD0CR")
            .field("olden", &self.olden())
            .field("thinb", &self.thinb())
            .field("bkold", &self.bkold())
            .field("acicn", &self.acicn())
            .field("acicd", &self.acicd())
            .field("oldactive", &self.oldactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - OLDx enable
    #[inline(always)]
    pub fn olden(&mut self) -> OLDEN_W<'_, OLD0CRrs> {
        OLDEN_W::new(self, 0)
    }
    ///Bit 1 - Threshold In band
    #[inline(always)]
    pub fn thinb(&mut self) -> THINB_W<'_, OLD0CRrs> {
        THINB_W::new(self, 1)
    }
    ///Bits 4:7 - Break signal assignment for out-of limit detector
    #[inline(always)]
    pub fn bkold(&mut self) -> BKOLD_W<'_, OLD0CRrs> {
        BKOLD_W::new(self, 4)
    }
    ///Bits 12:13 - OLDx CIC order selection
    #[inline(always)]
    pub fn acicn(&mut self) -> ACICN_W<'_, OLD0CRrs> {
        ACICN_W::new(self, 12)
    }
    ///Bits 17:21 - OLDx CIC decimation ratio selection
    #[inline(always)]
    pub fn acicd(&mut self) -> ACICD_W<'_, OLD0CRrs> {
        ACICD_W::new(self, 17)
    }
}
/**MDF out-of limit detector control register 0

You can [`read`](crate::Reg::read) this register and get [`old0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:OLD0CR)*/
pub struct OLD0CRrs;
impl crate::RegisterSpec for OLD0CRrs {
    type Ux = u32;
}
///`read()` method returns [`old0cr::R`](R) reader structure
impl crate::Readable for OLD0CRrs {}
///`write(|w| ..)` method takes [`old0cr::W`](W) writer structure
impl crate::Writable for OLD0CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OLD0CR to value 0
impl crate::Resettable for OLD0CRrs {}
