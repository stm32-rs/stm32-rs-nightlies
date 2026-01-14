///Register `DEM_MOD_DIG_USR` reader
pub type R = crate::R<DEM_MOD_DIG_USRrs>;
///Register `DEM_MOD_DIG_USR` writer
pub type W = crate::W<DEM_MOD_DIG_USRrs>;
///Field `CHANNEL_NUM` reader - Index for internal lock up table in which the synthesizer setup is contained.
pub type CHANNEL_NUM_R = crate::FieldReader;
///Field `CHANNEL_NUM` writer - Index for internal lock up table in which the synthesizer setup is contained.
pub type CHANNEL_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 1:7 - Index for internal lock up table in which the synthesizer setup is contained.
    #[inline(always)]
    pub fn channel_num(&self) -> CHANNEL_NUM_R {
        CHANNEL_NUM_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEM_MOD_DIG_USR")
            .field("channel_num", &self.channel_num())
            .finish()
    }
}
impl W {
    ///Bits 1:7 - Index for internal lock up table in which the synthesizer setup is contained.
    #[inline(always)]
    pub fn channel_num(&mut self) -> CHANNEL_NUM_W<'_, DEM_MOD_DIG_USRrs> {
        CHANNEL_NUM_W::new(self, 1)
    }
}
/**DEM_MOD_DIG_USR register

You can [`read`](crate::Reg::read) this register and get [`dem_mod_dig_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dem_mod_dig_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:DEM_MOD_DIG_USR)*/
pub struct DEM_MOD_DIG_USRrs;
impl crate::RegisterSpec for DEM_MOD_DIG_USRrs {
    type Ux = u32;
}
///`read()` method returns [`dem_mod_dig_usr::R`](R) reader structure
impl crate::Readable for DEM_MOD_DIG_USRrs {}
///`write(|w| ..)` method takes [`dem_mod_dig_usr::W`](W) writer structure
impl crate::Writable for DEM_MOD_DIG_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEM_MOD_DIG_USR to value 0x26
impl crate::Resettable for DEM_MOD_DIG_USRrs {
    const RESET_VALUE: u32 = 0x26;
}
