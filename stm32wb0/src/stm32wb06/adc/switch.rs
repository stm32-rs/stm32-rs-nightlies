///Register `SWITCH` reader
pub type R = crate::R<SWITCHrs>;
///Register `SWITCH` writer
pub type W = crate::W<SWITCHrs>;
///Field `SE_VIN_0` reader - input voltage for VINM\[0\] / VINP\[0\]-VINM\[0\]
pub type SE_VIN_0_R = crate::FieldReader;
///Field `SE_VIN_0` writer - input voltage for VINM\[0\] / VINP\[0\]-VINM\[0\]
pub type SE_VIN_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SE_VIN_1` reader - input voltage for VINM\[1\] / VINP\[1\]-VINM\[1\]
pub type SE_VIN_1_R = crate::FieldReader;
///Field `SE_VIN_1` writer - input voltage for VINM\[1\] / VINP\[1\]-VINM\[1\]
pub type SE_VIN_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SE_VIN_2` reader - input voltage for VINM\[2\] / VINP\[2\]-VINM\[2\]
pub type SE_VIN_2_R = crate::FieldReader;
///Field `SE_VIN_2` writer - input voltage for VINM\[2\] / VINP\[2\]-VINM\[2\]
pub type SE_VIN_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SE_VIN_3` reader - input voltage for VINM\[3\] / VINP\[3\]-VINM\[3\]
pub type SE_VIN_3_R = crate::FieldReader;
///Field `SE_VIN_3` writer - input voltage for VINM\[3\] / VINP\[3\]-VINM\[3\]
pub type SE_VIN_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SE_VIN_4` reader - input voltage for VINP\[0\]
pub type SE_VIN_4_R = crate::FieldReader;
///Field `SE_VIN_4` writer - input voltage for VINP\[0\]
pub type SE_VIN_4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SE_VIN_5` reader - input voltage for VINP\[1\]
pub type SE_VIN_5_R = crate::FieldReader;
///Field `SE_VIN_5` writer - input voltage for VINP\[1\]
pub type SE_VIN_5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SE_VIN_6` reader - input voltage for VINP\[2\]
pub type SE_VIN_6_R = crate::FieldReader;
///Field `SE_VIN_6` writer - input voltage for VINP\[2\]
pub type SE_VIN_6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SE_VIN_7` reader - input voltage for VINP\[3\]
pub type SE_VIN_7_R = crate::FieldReader;
///Field `SE_VIN_7` writer - input voltage for VINP\[3\]
pub type SE_VIN_7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - input voltage for VINM\[0\] / VINP\[0\]-VINM\[0\]
    #[inline(always)]
    pub fn se_vin_0(&self) -> SE_VIN_0_R {
        SE_VIN_0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - input voltage for VINM\[1\] / VINP\[1\]-VINM\[1\]
    #[inline(always)]
    pub fn se_vin_1(&self) -> SE_VIN_1_R {
        SE_VIN_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - input voltage for VINM\[2\] / VINP\[2\]-VINM\[2\]
    #[inline(always)]
    pub fn se_vin_2(&self) -> SE_VIN_2_R {
        SE_VIN_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - input voltage for VINM\[3\] / VINP\[3\]-VINM\[3\]
    #[inline(always)]
    pub fn se_vin_3(&self) -> SE_VIN_3_R {
        SE_VIN_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - input voltage for VINP\[0\]
    #[inline(always)]
    pub fn se_vin_4(&self) -> SE_VIN_4_R {
        SE_VIN_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - input voltage for VINP\[1\]
    #[inline(always)]
    pub fn se_vin_5(&self) -> SE_VIN_5_R {
        SE_VIN_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - input voltage for VINP\[2\]
    #[inline(always)]
    pub fn se_vin_6(&self) -> SE_VIN_6_R {
        SE_VIN_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - input voltage for VINP\[3\]
    #[inline(always)]
    pub fn se_vin_7(&self) -> SE_VIN_7_R {
        SE_VIN_7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWITCH")
            .field("se_vin_7", &self.se_vin_7())
            .field("se_vin_6", &self.se_vin_6())
            .field("se_vin_5", &self.se_vin_5())
            .field("se_vin_4", &self.se_vin_4())
            .field("se_vin_3", &self.se_vin_3())
            .field("se_vin_2", &self.se_vin_2())
            .field("se_vin_1", &self.se_vin_1())
            .field("se_vin_0", &self.se_vin_0())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - input voltage for VINM\[0\] / VINP\[0\]-VINM\[0\]
    #[inline(always)]
    pub fn se_vin_0(&mut self) -> SE_VIN_0_W<'_, SWITCHrs> {
        SE_VIN_0_W::new(self, 0)
    }
    ///Bits 2:3 - input voltage for VINM\[1\] / VINP\[1\]-VINM\[1\]
    #[inline(always)]
    pub fn se_vin_1(&mut self) -> SE_VIN_1_W<'_, SWITCHrs> {
        SE_VIN_1_W::new(self, 2)
    }
    ///Bits 4:5 - input voltage for VINM\[2\] / VINP\[2\]-VINM\[2\]
    #[inline(always)]
    pub fn se_vin_2(&mut self) -> SE_VIN_2_W<'_, SWITCHrs> {
        SE_VIN_2_W::new(self, 4)
    }
    ///Bits 6:7 - input voltage for VINM\[3\] / VINP\[3\]-VINM\[3\]
    #[inline(always)]
    pub fn se_vin_3(&mut self) -> SE_VIN_3_W<'_, SWITCHrs> {
        SE_VIN_3_W::new(self, 6)
    }
    ///Bits 8:9 - input voltage for VINP\[0\]
    #[inline(always)]
    pub fn se_vin_4(&mut self) -> SE_VIN_4_W<'_, SWITCHrs> {
        SE_VIN_4_W::new(self, 8)
    }
    ///Bits 10:11 - input voltage for VINP\[1\]
    #[inline(always)]
    pub fn se_vin_5(&mut self) -> SE_VIN_5_W<'_, SWITCHrs> {
        SE_VIN_5_W::new(self, 10)
    }
    ///Bits 12:13 - input voltage for VINP\[2\]
    #[inline(always)]
    pub fn se_vin_6(&mut self) -> SE_VIN_6_W<'_, SWITCHrs> {
        SE_VIN_6_W::new(self, 12)
    }
    ///Bits 14:15 - input voltage for VINP\[3\]
    #[inline(always)]
    pub fn se_vin_7(&mut self) -> SE_VIN_7_W<'_, SWITCHrs> {
        SE_VIN_7_W::new(self, 14)
    }
}
/**ADC switch control for Input Selection

You can [`read`](crate::Reg::read) this register and get [`switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#ADC:SWITCH)*/
pub struct SWITCHrs;
impl crate::RegisterSpec for SWITCHrs {
    type Ux = u32;
}
///`read()` method returns [`switch::R`](R) reader structure
impl crate::Readable for SWITCHrs {}
///`write(|w| ..)` method takes [`switch::W`](W) writer structure
impl crate::Writable for SWITCHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWITCH to value 0
impl crate::Resettable for SWITCHrs {}
