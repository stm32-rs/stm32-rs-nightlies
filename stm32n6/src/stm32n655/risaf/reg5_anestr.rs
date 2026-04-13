///Register `REG5_ANESTR` reader
pub type R = crate::R<REG5_ANESTRrs>;
///Register `REG5_ANESTR` writer
pub type W = crate::W<REG5_ANESTRrs>;
///Field `DCEN` reader - delegated configuration enable
pub type DCEN_R = crate::BitReader;
///Field `DCEN` writer - delegated configuration enable
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - delegated configuration enable
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG5_ANESTR")
            .field("dcen", &self.dcen())
            .finish()
    }
}
impl W {
    ///Bit 2 - delegated configuration enable
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W<'_, REG5_ANESTRrs> {
        DCEN_W::new(self, 2)
    }
}
/**RISAF region 5 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg5_anestr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_anestr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RISAF:REG5_ANESTR)*/
pub struct REG5_ANESTRrs;
impl crate::RegisterSpec for REG5_ANESTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg5_anestr::R`](R) reader structure
impl crate::Readable for REG5_ANESTRrs {}
///`write(|w| ..)` method takes [`reg5_anestr::W`](W) writer structure
impl crate::Writable for REG5_ANESTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG5_ANESTR to value 0
impl crate::Resettable for REG5_ANESTRrs {}
