///Register `REG13_BNESTR` reader
pub type R = crate::R<REG13_BNESTRrs>;
///Register `REG13_BNESTR` writer
pub type W = crate::W<REG13_BNESTRrs>;
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
        f.debug_struct("REG13_BNESTR")
            .field("dcen", &self.dcen())
            .finish()
    }
}
impl W {
    ///Bit 2 - delegated configuration enable
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W<'_, REG13_BNESTRrs> {
        DCEN_W::new(self, 2)
    }
}
/**RISAF region 13 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg13_bnestr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_bnestr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RISAF:REG13_BNESTR)*/
pub struct REG13_BNESTRrs;
impl crate::RegisterSpec for REG13_BNESTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg13_bnestr::R`](R) reader structure
impl crate::Readable for REG13_BNESTRrs {}
///`write(|w| ..)` method takes [`reg13_bnestr::W`](W) writer structure
impl crate::Writable for REG13_BNESTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG13_BNESTR to value 0
impl crate::Resettable for REG13_BNESTRrs {}
