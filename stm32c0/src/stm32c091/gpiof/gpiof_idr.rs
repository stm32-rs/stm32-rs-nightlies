///Register `GPIOF_IDR` reader
pub type R = crate::R<GPIOF_IDRrs>;
///Field `ID0` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID0_R = crate::BitReader;
///Field `ID1` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID1_R = crate::BitReader;
///Field `ID2` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID2_R = crate::BitReader;
///Field `ID3` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID3_R = crate::BitReader;
///Field `ID4` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID4_R = crate::BitReader;
///Field `ID5` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID5_R = crate::BitReader;
///Field `ID6` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID6_R = crate::BitReader;
///Field `ID7` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID7_R = crate::BitReader;
///Field `ID8` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID8_R = crate::BitReader;
///Field `ID9` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID9_R = crate::BitReader;
///Field `ID10` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID10_R = crate::BitReader;
///Field `ID11` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID11_R = crate::BitReader;
///Field `ID12` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID12_R = crate::BitReader;
///Field `ID13` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID13_R = crate::BitReader;
///Field `ID14` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID14_R = crate::BitReader;
///Field `ID15` reader - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
pub type ID15_R = crate::BitReader;
impl R {
    ///Bit 0 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id8(&self) -> ID8_R {
        ID8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id9(&self) -> ID9_R {
        ID9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id10(&self) -> ID10_R {
        ID10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id11(&self) -> ID11_R {
        ID11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id12(&self) -> ID12_R {
        ID12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id13(&self) -> ID13_R {
        ID13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id14(&self) -> ID14_R {
        ID14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x input data I/O y These bits are read-only. They contain the input value of the corresponding I/O port.
    #[inline(always)]
    pub fn id15(&self) -> ID15_R {
        ID15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOF_IDR")
            .field("id0", &self.id0())
            .field("id1", &self.id1())
            .field("id2", &self.id2())
            .field("id3", &self.id3())
            .field("id4", &self.id4())
            .field("id5", &self.id5())
            .field("id6", &self.id6())
            .field("id7", &self.id7())
            .field("id8", &self.id8())
            .field("id9", &self.id9())
            .field("id10", &self.id10())
            .field("id11", &self.id11())
            .field("id12", &self.id12())
            .field("id13", &self.id13())
            .field("id14", &self.id14())
            .field("id15", &self.id15())
            .finish()
    }
}
/**GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`gpiof_idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#GPIOF:GPIOF_IDR)*/
pub struct GPIOF_IDRrs;
impl crate::RegisterSpec for GPIOF_IDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpiof_idr::R`](R) reader structure
impl crate::Readable for GPIOF_IDRrs {}
///`reset()` method sets GPIOF_IDR to value 0
impl crate::Resettable for GPIOF_IDRrs {}
