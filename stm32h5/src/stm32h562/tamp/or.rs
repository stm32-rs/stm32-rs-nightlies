///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `OUT3_RMP` reader - TAMP_OUT3 mapping
pub type OUT3_RMP_R = crate::FieldReader;
///Field `OUT3_RMP` writer - TAMP_OUT3 mapping
pub type OUT3_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OUT5_RMP` reader - TAMP_OUT5 mapping
pub type OUT5_RMP_R = crate::BitReader;
///Field `OUT5_RMP` writer - TAMP_OUT5 mapping
pub type OUT5_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN2_RMP` reader - TAMP_IN2 mapping
pub type IN2_RMP_R = crate::BitReader;
///Field `IN2_RMP` writer - TAMP_IN2 mapping
pub type IN2_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN3_RMP` reader - TAMP_IN3 mapping
pub type IN3_RMP_R = crate::BitReader;
///Field `IN3_RMP` writer - TAMP_IN3 mapping
pub type IN3_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN4_RMP` reader - TAMP_IN4 mapping
pub type IN4_RMP_R = crate::BitReader;
///Field `IN4_RMP` writer - TAMP_IN4 mapping
pub type IN4_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 1:2 - TAMP_OUT3 mapping
    #[inline(always)]
    pub fn out3_rmp(&self) -> OUT3_RMP_R {
        OUT3_RMP_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - TAMP_OUT5 mapping
    #[inline(always)]
    pub fn out5_rmp(&self) -> OUT5_RMP_R {
        OUT5_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - TAMP_IN2 mapping
    #[inline(always)]
    pub fn in2_rmp(&self) -> IN2_RMP_R {
        IN2_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TAMP_IN3 mapping
    #[inline(always)]
    pub fn in3_rmp(&self) -> IN3_RMP_R {
        IN3_RMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TAMP_IN4 mapping
    #[inline(always)]
    pub fn in4_rmp(&self) -> IN4_RMP_R {
        IN4_RMP_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("out3_rmp", &self.out3_rmp())
            .field("out5_rmp", &self.out5_rmp())
            .field("in2_rmp", &self.in2_rmp())
            .field("in3_rmp", &self.in3_rmp())
            .field("in4_rmp", &self.in4_rmp())
            .finish()
    }
}
impl W {
    ///Bits 1:2 - TAMP_OUT3 mapping
    #[inline(always)]
    pub fn out3_rmp(&mut self) -> OUT3_RMP_W<'_, ORrs> {
        OUT3_RMP_W::new(self, 1)
    }
    ///Bit 3 - TAMP_OUT5 mapping
    #[inline(always)]
    pub fn out5_rmp(&mut self) -> OUT5_RMP_W<'_, ORrs> {
        OUT5_RMP_W::new(self, 3)
    }
    ///Bit 8 - TAMP_IN2 mapping
    #[inline(always)]
    pub fn in2_rmp(&mut self) -> IN2_RMP_W<'_, ORrs> {
        IN2_RMP_W::new(self, 8)
    }
    ///Bit 9 - TAMP_IN3 mapping
    #[inline(always)]
    pub fn in3_rmp(&mut self) -> IN3_RMP_W<'_, ORrs> {
        IN3_RMP_W::new(self, 9)
    }
    ///Bit 10 - TAMP_IN4 mapping
    #[inline(always)]
    pub fn in4_rmp(&mut self) -> IN4_RMP_W<'_, ORrs> {
        IN4_RMP_W::new(self, 10)
    }
}
/**TAMP option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#TAMP:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
