///Register `DX1DQSTR` reader
pub type R = crate::R<DX1DQSTRrs>;
///Register `DX1DQSTR` writer
pub type W = crate::W<DX1DQSTRrs>;
///Field `R0DGSL` reader - R0DGSL
pub type R0DGSL_R = crate::FieldReader;
///Field `R0DGSL` writer - R0DGSL
pub type R0DGSL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `R0DGPS` reader - R0DGPS
pub type R0DGPS_R = crate::FieldReader;
///Field `R0DGPS` writer - R0DGPS
pub type R0DGPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DQSDLY` reader - DQSDLY
pub type DQSDLY_R = crate::FieldReader;
///Field `DQSDLY` writer - DQSDLY
pub type DQSDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DQSNDLY` reader - DQSNDLY
pub type DQSNDLY_R = crate::FieldReader;
///Field `DQSNDLY` writer - DQSNDLY
pub type DQSNDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DMDLY` reader - DMDLY
pub type DMDLY_R = crate::FieldReader;
///Field `DMDLY` writer - DMDLY
pub type DMDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:2 - R0DGSL
    #[inline(always)]
    pub fn r0dgsl(&self) -> R0DGSL_R {
        R0DGSL_R::new((self.bits & 7) as u8)
    }
    ///Bits 12:13 - R0DGPS
    #[inline(always)]
    pub fn r0dgps(&self) -> R0DGPS_R {
        R0DGPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 20:22 - DQSDLY
    #[inline(always)]
    pub fn dqsdly(&self) -> DQSDLY_R {
        DQSDLY_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25 - DQSNDLY
    #[inline(always)]
    pub fn dqsndly(&self) -> DQSNDLY_R {
        DQSNDLY_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bits 26:29 - DMDLY
    #[inline(always)]
    pub fn dmdly(&self) -> DMDLY_R {
        DMDLY_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DX1DQSTR")
            .field("r0dgsl", &self.r0dgsl())
            .field("r0dgps", &self.r0dgps())
            .field("dqsdly", &self.dqsdly())
            .field("dqsndly", &self.dqsndly())
            .field("dmdly", &self.dmdly())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - R0DGSL
    #[inline(always)]
    pub fn r0dgsl(&mut self) -> R0DGSL_W<DX1DQSTRrs> {
        R0DGSL_W::new(self, 0)
    }
    ///Bits 12:13 - R0DGPS
    #[inline(always)]
    pub fn r0dgps(&mut self) -> R0DGPS_W<DX1DQSTRrs> {
        R0DGPS_W::new(self, 12)
    }
    ///Bits 20:22 - DQSDLY
    #[inline(always)]
    pub fn dqsdly(&mut self) -> DQSDLY_W<DX1DQSTRrs> {
        DQSDLY_W::new(self, 20)
    }
    ///Bits 23:25 - DQSNDLY
    #[inline(always)]
    pub fn dqsndly(&mut self) -> DQSNDLY_W<DX1DQSTRrs> {
        DQSNDLY_W::new(self, 23)
    }
    ///Bits 26:29 - DMDLY
    #[inline(always)]
    pub fn dmdly(&mut self) -> DMDLY_W<DX1DQSTRrs> {
        DMDLY_W::new(self, 26)
    }
}
/**DDRPHYC byte lane 1 DQST register

You can [`read`](crate::Reg::read) this register and get [`dx1dqstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx1dqstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX1DQSTR)*/
pub struct DX1DQSTRrs;
impl crate::RegisterSpec for DX1DQSTRrs {
    type Ux = u32;
}
///`read()` method returns [`dx1dqstr::R`](R) reader structure
impl crate::Readable for DX1DQSTRrs {}
///`write(|w| ..)` method takes [`dx1dqstr::W`](W) writer structure
impl crate::Writable for DX1DQSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DX1DQSTR to value 0x3db0_2000
impl crate::Resettable for DX1DQSTRrs {
    const RESET_VALUE: u32 = 0x3db0_2000;
}
