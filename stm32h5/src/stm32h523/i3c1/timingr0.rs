///Register `TIMINGR0` reader
pub type R = crate::R<TIMINGR0rs>;
///Register `TIMINGR0` writer
pub type W = crate::W<TIMINGR0rs>;
///Field `SCLL_PP` reader - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles:
pub type SCLL_PP_R = crate::FieldReader;
///Field `SCLL_PP` writer - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles:
pub type SCLL_PP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLH_I3C` reader - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles:
pub type SCLH_I3C_R = crate::FieldReader;
///Field `SCLH_I3C` writer - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles:
pub type SCLH_I3C_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLL_OD` reader - SCL low duration in open-drain phases, used for legacy Iless thansup>2less than/sup>C messages and for I3C open-drain phases (address phase following a start, ACK phase during controller-initiated messages, and T bit phase during direct/private/IBI payload), in number of kernel clocks cycles:
pub type SCLL_OD_R = crate::FieldReader;
///Field `SCLL_OD` writer - SCL low duration in open-drain phases, used for legacy Iless thansup>2less than/sup>C messages and for I3C open-drain phases (address phase following a start, ACK phase during controller-initiated messages, and T bit phase during direct/private/IBI payload), in number of kernel clocks cycles:
pub type SCLL_OD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLH_I2C` reader - SCL high duration, used for legacy Iless thansup>2less than/sup>C messages, in number of kernel clocks cycles:
pub type SCLH_I2C_R = crate::FieldReader;
///Field `SCLH_I2C` writer - SCL high duration, used for legacy Iless thansup>2less than/sup>C messages, in number of kernel clocks cycles:
pub type SCLH_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles:
    #[inline(always)]
    pub fn scll_pp(&self) -> SCLL_PP_R {
        SCLL_PP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles:
    #[inline(always)]
    pub fn sclh_i3c(&self) -> SCLH_I3C_R {
        SCLH_I3C_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - SCL low duration in open-drain phases, used for legacy Iless thansup>2less than/sup>C messages and for I3C open-drain phases (address phase following a start, ACK phase during controller-initiated messages, and T bit phase during direct/private/IBI payload), in number of kernel clocks cycles:
    #[inline(always)]
    pub fn scll_od(&self) -> SCLL_OD_R {
        SCLL_OD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - SCL high duration, used for legacy Iless thansup>2less than/sup>C messages, in number of kernel clocks cycles:
    #[inline(always)]
    pub fn sclh_i2c(&self) -> SCLH_I2C_R {
        SCLH_I2C_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMINGR0")
            .field("scll_pp", &self.scll_pp())
            .field("sclh_i3c", &self.sclh_i3c())
            .field("scll_od", &self.scll_od())
            .field("sclh_i2c", &self.sclh_i2c())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles:
    #[inline(always)]
    pub fn scll_pp(&mut self) -> SCLL_PP_W<'_, TIMINGR0rs> {
        SCLL_PP_W::new(self, 0)
    }
    ///Bits 8:15 - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles:
    #[inline(always)]
    pub fn sclh_i3c(&mut self) -> SCLH_I3C_W<'_, TIMINGR0rs> {
        SCLH_I3C_W::new(self, 8)
    }
    ///Bits 16:23 - SCL low duration in open-drain phases, used for legacy Iless thansup>2less than/sup>C messages and for I3C open-drain phases (address phase following a start, ACK phase during controller-initiated messages, and T bit phase during direct/private/IBI payload), in number of kernel clocks cycles:
    #[inline(always)]
    pub fn scll_od(&mut self) -> SCLL_OD_W<'_, TIMINGR0rs> {
        SCLL_OD_W::new(self, 16)
    }
    ///Bits 24:31 - SCL high duration, used for legacy Iless thansup>2less than/sup>C messages, in number of kernel clocks cycles:
    #[inline(always)]
    pub fn sclh_i2c(&mut self) -> SCLH_I2C_W<'_, TIMINGR0rs> {
        SCLH_I2C_W::new(self, 24)
    }
}
/**I3C timing register 0

You can [`read`](crate::Reg::read) this register and get [`timingr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#I3C1:TIMINGR0)*/
pub struct TIMINGR0rs;
impl crate::RegisterSpec for TIMINGR0rs {
    type Ux = u32;
}
///`read()` method returns [`timingr0::R`](R) reader structure
impl crate::Readable for TIMINGR0rs {}
///`write(|w| ..)` method takes [`timingr0::W`](W) writer structure
impl crate::Writable for TIMINGR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMINGR0 to value 0
impl crate::Resettable for TIMINGR0rs {}
